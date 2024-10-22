//use core::ptr::addr_of_mut;
use da14531_sdk::{
    app_modules::{app_common::app::app_prf_enable, app_env_get_conidx, register_app_callbacks},
    bindings::{default_app_on_disconnect, default_app_on_init},
    ble_stack::{
        host::gap::{
            gapc::task::{GapcConnectionReqInd, GapcDisconnectInd},
            GAP_INVALID_CONIDX,
        },
        //rwble_hl::error::HlErr::GAP_ERR_CANCELED,
    },
    platform::{arch::register_main_loop_callbacks, core_modules::crypto::aes_init},
    register_user_operation_adv,
};
use rtt_target::{rtt_init_print, ChannelMode::NoBlockSkip};

use crate::{app::App, ble::Da14531Ble, peripherals::Da14531Peripherals};

/// Defines the `Da14531App` for convenience
type Da14531App = App<Da14531Peripherals, Da14531Ble>;

/// The actual instance of the app struct
static mut APP: Da14531App = Da14531App::new();

/// Get a mutable reference to the app
/// # Safety
///
/// This must never be called outside the main thread
#[allow(static_mut_refs)]
pub unsafe fn app() -> &'static mut Da14531App {
    &mut APP
}

/// Initialize peripherals
#[no_mangle]
pub extern "C" fn periph_init() {
    rtt_init_print!(NoBlockSkip, 640);

    unsafe { app() }.init_peripherals();
}

// Register handler for `default_operation_adv` as default app operation
register_user_operation_adv!(app_advertising_start_callback);

/// Trigger advertising in app
#[inline]
fn app_advertising_start_callback() {
    unsafe { app() }.on_start_advertising();
}
// Register the app_on_init handler
register_main_loop_callbacks! {
    app_on_init: app_on_init_callback,
}

/// Initialize AES and run `default_app_on_init` from SDK
#[inline]
pub fn app_on_init_callback() {
    aes_init(false);
    unsafe { app() }.init();

    unsafe { default_app_on_init() };
}

// Register app callback handlers
register_app_callbacks! {
    app_on_connection: user_app_connection,
    app_on_adv_undirect_complete: user_app_adv_undirect_complete,
    app_on_disconnect: user_app_disconnect,
}

#[inline]
fn user_app_connection(conidx: u8, _param: &GapcConnectionReqInd) {
    if app_env_get_conidx(conidx) != GAP_INVALID_CONIDX as u8 {
        app_prf_enable(conidx);

        unsafe { app() }.on_connect(Some(conidx as u32));
    } else {
        unsafe { app() }.on_connect(None);
    }
}

#[inline]
fn user_app_disconnect(_param: &GapcDisconnectInd) {
    unsafe { default_app_on_disconnect(core::ptr::null()) };

    unsafe { app() }.on_disconnect();
}
