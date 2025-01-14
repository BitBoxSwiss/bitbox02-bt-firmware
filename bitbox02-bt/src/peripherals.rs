use da14531_hal::{
    //crg_top::{CrgTop, CrgTopExt},
    //nvic::{Nvic, NvicExt},
    //crg_aon::CrgAonExt,
    //cm::{peripheral::SCB, Peripherals as CmPeripherals},
    crg_aon::{CrgAon, CrgAonExt},
    pac::Peripherals,
    sys_wdog::{SysWdog, SysWdogExt},
};
use da14531_sdk::bindings::{GPIO_FUNCTION::*, GPIO_PUPD_INPUT, GPIO_PUPD_OUTPUT};
use da14531_sdk::platform::{
    driver::{
        gpio::GPIO_ConfigurePin,
        syscntl::{dcdc_turn_on_in_boost, SyscntlDcdcLevel::SYSCNTL_DCDC_LEVEL_3V0},
        uart,
    },
    system_library::patch_func,
};

use crate::app::PeripheralsDriver;

/// This struct contains all relevant peripherals and implements the `PeripheralsDriver` trait
pub struct Da14531Peripherals {
    sys_wdog: SysWdog,
    //nvic: Nvic,
    //scb: SCB,
    crg_aon: CrgAon,
    //crg_top: CrgTop,
}

// Peripherals contains PhantomData<*const ()>, which isn't Sync. As long as this type is never
// used in an ISR it is safe to implement Sync.
unsafe impl Sync for Da14531Peripherals {}

impl Da14531Peripherals {
    pub fn new() -> Self {
        dcdc_turn_on_in_boost(SYSCNTL_DCDC_LEVEL_3V0);

        patch_func();

        let dp = Peripherals::take().unwrap();
        //let cp = CmPeripherals::take().unwrap();

        // Get necessary peripherals
        let sys_wdog = dp.SYS_WDOG.constrain();
        //let nvic = cp.NVIC.constrain();
        //let scb = cp.SCB;
        //let crg_top = dp.CRG_TOP.constrain();
        let mut crg_aon = dp.CRG_AON.constrain();

        // Disable haradware reset, needed beacuse the pin is reused for Uart TX
        crg_aon.hardware_reset_dis(true);

        // Initialize uart
        uart::init();

        GPIO_ConfigurePin(0, GPIO_PUPD_OUTPUT, PID_UART1_TX, false);
        GPIO_ConfigurePin(1, GPIO_PUPD_INPUT, PID_UART1_RX, false);
        GPIO_ConfigurePin(3, GPIO_PUPD_INPUT, PID_UART1_CTSN, false);
        GPIO_ConfigurePin(4, GPIO_PUPD_OUTPUT, PID_UART1_RTSN, false);

        // Enable pad latch
        crg_aon.set_pad_latch_en(true);

        Da14531Peripherals {
            sys_wdog,
            crg_aon,
            //uart,
            //nvic,
            //crg_top,
            //scb,
        }
    }
}

impl Default for Da14531Peripherals {
    fn default() -> Self {
        Self::new()
    }
}

impl PeripheralsDriver for Da14531Peripherals {
    fn new() -> Self {
        Self::new()
    }

    /// Feed the dog :)
    fn feed_watchdog(&mut self) {
        self.sys_wdog.feed();
    }
}
