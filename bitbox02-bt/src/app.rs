use core::marker::PhantomData;
use da14531_sdk::app_modules::configure_device_information_service;
use da14531_sdk::app_modules::timer::AppTimer;
use da14531_sdk::ble_stack::profiles::custom::custs::custs1::task::KeMsgDynCusts1ValIndReq;
use da14531_sdk::ble_stack::profiles::prf::prf_get_task_from_id;
use da14531_sdk::ble_stack::rwble_hl::error::HlErr::GAP_ERR_NO_ERROR as ATT_ERR_NO_ERROR;
use da14531_sdk::platform::core_modules::rwip::{KeApiId::TASK_ID_CUSTS1, KeTaskType::TASK_APP};
use da14531_sdk::platform::driver::uart;
use grounded::const_init::ConstInit;
use rtt_target::{rprint, rprintln};

/// Defines an interface to access the peripherals
pub trait PeripheralsDriver {
    fn new() -> Self;
    fn feed_watchdog(&mut self);
}

/// Defines an interface to control the BLE stack
pub trait BleDriver {
    fn start_adverstising();
    fn stop_adverstising();
    fn disconnect(connection_handle: u8);
}

/// Holds the state of the application
pub struct App<P, BLE>
where
    Self: 'static,
    P: 'static + PeripheralsDriver,
    BLE: 'static + BleDriver,
{
    peripherals: Option<P>,
    connection_handle: Option<u8>,
    _ble: PhantomData<BLE>,
}

impl<P, BLE> Default for App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P, BLE> grounded::const_init::ConstInit for App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    const VAL: Self = Self::new();
}

fn change_product_char() {
    static mut COUNTER: usize = 0;
    const PRODUCT: &[&[u8]] = &[
        b"Bitbox02p",
        b"no connection",
        b"bb02p-bootloader",
        b"no connection",
    ];

    AppTimer::new(500, || {
        let mut update = KeMsgDynCusts1ValIndReq::<64>::new(
            TASK_APP as u16,
            prf_get_task_from_id(TASK_ID_CUSTS1 as u16),
        );
        update.fields().conidx = 0;
        update.fields().handle = crate::ble::config::char_idx_map::CHAR_PRODUCT_HANDLE;
        let counter = unsafe { &mut COUNTER };
        let len = PRODUCT[*counter % PRODUCT.len()].len();

        update.fields().length = len as u16;

        let value = unsafe { update.fields().value.as_mut_slice(len) };
        value[..len].copy_from_slice(PRODUCT[*counter % PRODUCT.len()]);

        rprintln!("updating with {}", unsafe {
            core::str::from_utf8_unchecked(PRODUCT[*counter % PRODUCT.len()])
        });

        update.send();

        *counter += 1;

        change_product_char()
    });
}

/// Business logic of the application
impl<P, BLE> App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    /// Create new instance of App
    pub const fn new() -> Self {
        Self {
            peripherals: None,
            _ble: PhantomData,
            connection_handle: None,
        }
    }

    /// Initialize peripherals
    // Never, ever, ever, try to allocate in this function.
    pub fn init_peripherals(&mut self) {
        rprint!("Initializing peripherals...");
        self.peripherals = Some(P::new());
        rprintln!("done!");
    }

    /// On app init called before main event loop
    pub fn init(&mut self) {
        rprintln!("init");
    }

    /// On db init complete is called when the ble services db is initialized
    pub fn on_set_dev_config_complete(&mut self) {
        rprintln!("on_set_dev_config_complete");
        change_product_char();
    }

    /// On db init complete is called when the ble services db is initialized
    pub fn on_db_init_complete(&mut self) {
        rprintln!("on_db_init_complete");
        //change_product_char();
    }

    /// Start advertising handler
    pub fn on_start_advertising(&mut self) {
        let state = unsafe { da14531_sdk::bindings::ke_state_get(TASK_APP as u16) };
        rprintln!("App::on_start_advertising() {}", state);

        BLE::start_adverstising();
    }

    /// Connect event handler
    pub fn on_connect(&mut self, connection_handle: Option<u8>) {
        let state = unsafe { da14531_sdk::bindings::ke_state_get(TASK_APP as u16) };
        self.connection_handle = connection_handle;
        rprintln!("on_connect id: {} {}", connection_handle.unwrap(), state);
    }

    /// Disonnect event handler
    pub fn on_disconnect(&mut self) {
        rprintln!("on_disconnect");
        self.connection_handle = None;
    }

    pub fn feed_watchdog(&mut self) {
        self.peripherals().feed_watchdog();
    }

    pub fn peripherals(&mut self) -> &mut P {
        self.peripherals.as_mut().unwrap()
    }
}

#[no_mangle]
static USER_DEVICE_NAME: &str = "BitBox02+";

#[no_mangle]
static USER_DEVICE_APPEARANCE: u16 = 0x0240; // Keyring appearance
