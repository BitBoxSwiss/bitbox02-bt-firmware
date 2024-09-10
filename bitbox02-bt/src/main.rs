#![feature(core_intrinsics)]
#![allow(internal_features)]
#![no_std]
#![no_main]
extern crate alloc;

//#[cfg(debug_assertions)]
//use panic_halt as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use da14531_sdk::allocator::Da14531Allocator;

/// The actual application code and definition of interfaces for peripheral and BLE drivers
pub mod app;
/// Glue between SDK system and application code
pub mod app_impl;
/// BLE
pub mod ble;
/// HAL for peripherals
pub mod peripherals;

#[global_allocator]
static ALLOCATOR: Da14531Allocator = Da14531Allocator;

#[cfg(debug_assertions)]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    use portable_atomic::{compiler_fence, Ordering};
    use rtt_target::UpChannel;

    // From now on we don't need interrupts. Restarting the MCU is the only way to get out of this
    // panic handler.
    cortex_m::interrupt::disable();

    // Calling conjure is only safe if we have disabled interrupts, which we have.
    let Some(mut ch) = (unsafe { UpChannel::conjure(0) }) else {
        core::intrinsics::abort();
    };

    // Bringing in all the formatting functionality takes a lot of space, print "panic, halted"
    // instead for now.
    //let Ok(()) = writeln!(ch, "{}", info) else {
    let Ok(()) = writeln!(ch, "panic, halted") else {
        core::intrinsics::abort();
    };

    loop {
        // Feeding the watchdog is implemented using the PAC (direct register access) instead of
        // the HAL layer. This makes it possible to panic even before the HAL is initialized.
        let sys_wdog = unsafe { &*da14531_hal::pac::SYS_WDOG::ptr() };
        sys_wdog.watchdog_reg.modify(|_, w| unsafe {
            w.wdog_val()
                .bits(da14531_hal::sys_wdog::WATCHDOG_DEFAULT_PERIOD)
        });
        compiler_fence(Ordering::SeqCst);
    }
}
