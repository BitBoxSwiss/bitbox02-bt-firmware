use core::marker::PhantomData;
use da14531_sdk::app_modules::configure_device_information_service;
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

    /// On app init
    pub fn init(&mut self) {
        rprintln!("init");
    }

    /// Start advertising handler
    pub fn on_start_advertising(&mut self) {
        rprintln!("App::on_start_advertising()");

        BLE::start_adverstising();
    }

    /// Connect event handler
    pub fn on_connect(&mut self, connection_handle: Option<u8>) {
        self.connection_handle = connection_handle;
        rprintln!("on_connect id: {}", connection_handle.unwrap());
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

configure_device_information_service! {
    manufacturer_name: "BitBox Swiss"
}

#[no_mangle]
static USER_DEVICE_NAME: &str = "";

#[no_mangle]
static USER_DEVICE_APPEARANCE: u16 = 0x0240; // Keyring appearance
