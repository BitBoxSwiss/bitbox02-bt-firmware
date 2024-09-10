//use alloc::boxed::Box;
use core::marker::PhantomData;

use rtt_target::{rprint, rprintln};

use da14531_sdk::app_modules::configure_device_information_service;
//use da14531_sdk::app_modules::timer::AppTimer;

// /// Stop advertising and go to hibernation after X secs
// const APP_ADVERTISTE_STOP_TIMER: u32 = 60;

/// Defines an interface to access the peripherals
pub trait PeripheralsDriver {
    fn new() -> Self;
    fn feed_watchdog(&mut self);
}

/// Defines an interface to control the BLE stack
pub trait BleDriver {
    fn start_adverstising();
    fn stop_adverstising();
    fn disconnect(connection_handle: u32);
}

/// Holds the state of the application
pub struct App<P, BLE>
where
    Self: 'static,
    P: 'static + PeripheralsDriver,
    BLE: 'static + BleDriver,
{
    peripherals: Option<P>,
    connection_handle: Option<u32>,
    _ble: PhantomData<BLE>,
    //timer: Option<AppTimer>,
}

//fn timer_cb() {
//    rprintln!("tick")
//}

impl<P, BLE> Default for App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    fn default() -> Self {
        Self::new()
    }
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
            //timer: None,
        }
    }

    /// Initialize peripherals
    // Never, ever, ever, try to allocate in this function.
    pub fn init_peripherals(&mut self) {
        #![allow(non_upper_case_globals)]

        //rprintln!("heap sizes: {}");

        //extern "C" {
        //    static __StackTop: u32;
        //    static __HeapBase: u32;
        //    static __HeapLimit: u32;
        //    static __heap_mem_area_not_ret_start__: u32;
        //    static __heap_mem_area_not_ret_end__: u32;
        //    static __db_heap_start__: u32;
        //    static __db_heap_end__: u32;
        //    static rwip_heap_non_ret: u32;
        //    static rwip_heap_env_ret: u32;
        //    static rwip_heap_msg_ret: u32;
        //    static rwip_heap_db_ret: u32;
        //}

        //unsafe {
        //    //let initial_sp: *const u32 = &__StackTop;
        //    //let heap_base: *const u32 = &__HeapBase;
        //    //let heap_limit: *const u32 = &__HeapLimit;
        //    let not_ret_start: *const u32 = &__heap_mem_area_not_ret_start__;
        //    let not_ret_end: *const u32 = &__heap_mem_area_not_ret_end__;
        //    let db_heap_start: *const u32 = &__db_heap_start__;
        //    let db_heap_end: *const u32 = &__db_heap_end__;

        //    //rprintln!("Compiled: %s %s", __DATE__, __TIME__);
        //    //rprintln!("stack: {:?}", initial_sp);
        //    //rprintln!("heap: {:?} ({:?})", heap_base, heap_limit);
        //    rprintln!(
        //        "__heap_mem_area_not_ret_start__:  {:?}, __heap_mem_area_not_ret_end__: {:?}, size in bytes {}",
        //        not_ret_start,
        //        not_ret_end,
        //        4 * (not_ret_end.offset_from(not_ret_start)) as u32
        //    );
        //    rprintln!("NON_RET_HEAP_SZ {}", da14531_sdk::bindings::NON_RET_HEAP_SZ);
        //    rprintln!(
        //        "db_heap: {:?}:{:?} size {}",
        //        db_heap_start,
        //        db_heap_end,
        //        4 * (db_heap_end.offset_from(db_heap_start)) as u32
        //    );
        //    let a = Box::new(1);
        //    //rprintln!(
        //    //    "rwip retained {}",
        //    //    /*da14531_sdk::bindings::MSG_HEAP_SZ + da14531_sdk::bindings::ENV_HEAP_SZ + da14531_sdk::bindings::DB_HEAP_SZ */;
        //    //);
        //}
        rprint!("Initializing peripherals...");
        self.peripherals = Some(P::new());
        rprintln!("done!");
    }

    /// On app init
    pub fn init(&mut self) {
        rprint!("Initializing timer...");
        //let cb = Box::new(|| critical_section::with(|_| rprintln!("tick")));
        //let period = 1 * 100; /* 1s, unit is 10ms */
        //let timer = AppTimer::new(1, Box::new(timer_cb)).unwrap();
        //timer.start();
        //self.timer = Some(timer);
        rprintln!("done!");
    }

    /// Start advertising handler
    pub fn on_start_advertising(&mut self) {
        rprintln!("App::on_start_advertising()");

        BLE::start_adverstising();
    }

    /// Connect event handler
    pub fn on_connect(&mut self, connection_handle: Option<u32>) {
        rprintln!("on_connect");
        self.connection_handle = connection_handle;
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
pub static USER_DEVICE_NAME: &str = "bitbox";
