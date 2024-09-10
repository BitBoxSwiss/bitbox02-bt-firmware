use da14531_hal::{
    //cm::{peripheral::SCB, Peripherals as CmPeripherals},
    //crg_aon::{CrgAon, CrgAonExt},
    //crg_top::{CrgTop, CrgTopExt},
    //nvic::{Nvic, NvicExt},
    crg_aon::CrgAonExt,
    pac::Peripherals,
    sys_wdog::{SysWdog, SysWdogExt},
};
use da14531_sdk::platform::{
    driver::syscntl::{dcdc_turn_on_in_boost, SyscntlDcdcLevel::SYSCNTL_DCDC_LEVEL_3V0},
    system_library::patch_func,
};

use crate::app::PeripheralsDriver;

/// This struct contains all relevant peripherals and implements the `PeripheralsDriver` trait
pub struct Da14531Peripherals {
    sys_wdog: SysWdog,
    //nvic: Nvic,
    //scb: SCB,
    //crg_aon: CrgAon,
    //crg_top: CrgTop,
}

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

        // Enable pad latch
        crg_aon.set_pad_latch_en(true);

        Da14531Peripherals {
            sys_wdog,
            //nvic,
            //crg_aon,
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
