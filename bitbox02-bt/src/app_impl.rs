//use core::ptr::addr_of_mut;
use da14531_sdk::{
    app_modules::{app_common::app::app_prf_enable, app_env_get_conidx, register_app_callbacks},
    bindings::{default_app_on_disconnect, default_app_on_init},
    ble_stack::host::gap::{
        gapc::task::{GapcConnectionReqInd, GapcDisconnectInd},
        GAP_INVALID_CONIDX,
    },
    platform::{arch::register_main_loop_callbacks, core_modules::crypto::aes_init},
    register_user_operation_adv,
};
use grounded::uninit::GroundedCell;
use rtt_target::{rtt_init_print, ChannelMode::NoBlockSkip};

use crate::{app::App, ble::Da14531Ble, peripherals::Da14531Peripherals};

/// Defines the `Da14531App` for convenience
type Da14531App = App<Da14531Peripherals, Da14531Ble>;

/// The actual instance of the app struct
static APP: GroundedCell<Da14531App> = GroundedCell::const_init();

/// Initialize peripherals
#[no_mangle]
pub extern "C" fn periph_init() {
    let app = unsafe { &mut *APP.get() };

    rtt_init_print!(NoBlockSkip, 640);

    app.init_peripherals();
}

// Register handler for `default_operation_adv` as default app operation
register_user_operation_adv!(app_advertising_start_callback);

/// Trigger advertising in app
#[inline]
fn app_advertising_start_callback() {
    let app = unsafe { &mut *APP.get() };

    app.on_start_advertising();
}
// Register the app_on_init handler
register_main_loop_callbacks! {
    app_on_init: app_on_init_callback,
}

/// Initialize AES and run `default_app_on_init` from SDK
#[inline]
pub fn app_on_init_callback() {
    let app = unsafe { &mut *APP.get() };

    aes_init(false);
    app.init();

    // `default_app_on_init()` initializes app SDK apps that are enabled
    unsafe { default_app_on_init() };
}

// Register app callback handlers
register_app_callbacks! {
    app_on_connection: user_app_connection,
    app_on_disconnect: user_app_disconnect,
}

#[inline]
fn user_app_connection(conidx: u8, _param: &GapcConnectionReqInd) {
    let app = unsafe { &mut *APP.get() };

    if app_env_get_conidx(conidx) != GAP_INVALID_CONIDX as u8 {
        // Enable the created profiles/services
        app_prf_enable(conidx);

        // Port this if security is enabled (copied from default handler)
        //if user_default_hnd_conf.security_request_scenario == DEF_SEC_REQ_ON_CONNECT {
        //    app_easy_security_request(conidx)
        //}

        app.on_connect(Some(conidx));
    } else {
        // No connection has been established
        app.on_connect(None);
    }
    // We want to manage advertise on/off by ourselves. Therefore we do not call
    // `default_app_on_connection`
}

#[inline]
fn user_app_disconnect(_param: &GapcDisconnectInd) {
    // `default_app_on_disconnect` calls default_operation_adv to start advertising again
    unsafe { default_app_on_disconnect(core::ptr::null()) };
    let app = unsafe { &mut *APP.get() };

    app.on_disconnect();
}
