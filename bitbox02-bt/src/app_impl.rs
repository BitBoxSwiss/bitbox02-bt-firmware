//use core::ptr::addr_of_mut;
use da14531_sdk::bindings::{
    app_easy_security_request, gap_auth_mask_GAP_AUTH_BOND, gap_io_cap_GAP_IO_CAP_DISPLAY_YES_NO,
    gap_kdist_GAP_KDIST_ENCKEY, gap_kdist_GAP_KDIST_IDKEY, gap_kdist_GAP_KDIST_SIGNKEY,
    gap_oob_GAP_OOB_AUTH_DATA_NOT_PRESENT, gap_sec_req_GAP_SEC2_AUTH_DATA_SGN,
    gap_tk_type_GAP_TK_KEY_CONFIRM,
};
use da14531_sdk::{
    app_modules::{
        app_common::app::app_prf_enable, app_env_get_conidx, register_app_bond_db_callbacks,
        register_app_callbacks,
    },
    bindings::{
        app_easy_security_bdb_init, app_prf_srv_perm_SRV_PERM_AUTH, app_set_prf_srv_perm,
        default_app_on_disconnect, default_app_on_init, default_app_on_set_dev_config_complete,
        default_app_on_tk_exch,
    },
    ble_stack::host::gap::{
        gapc::task::{GapcConnectionReqInd, GapcDisconnectInd},
        GAP_INVALID_CONIDX,
    },
    platform::core_modules::rwip::KeApiId::TASK_ID_CUSTS1,
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
    //unsafe { app_set_prf_srv_perm(TASK_ID_CUSTS1, app_prf_srv_perm_SRV_PERM_SECURE) };
    unsafe { app_set_prf_srv_perm(TASK_ID_CUSTS1, app_prf_srv_perm_SRV_PERM_AUTH) };
    // Fetch bond data from the external memory
    unsafe { app_easy_security_bdb_init() }
}

// Register app callback handlers
register_app_callbacks! {
    app_on_connection: user_app_connection,
    app_on_db_init_complete: user_app_db_init_complete,
    app_on_set_dev_config_complete: user_app_on_set_dev_config_complete,
    app_on_disconnect: user_app_disconnect,
    app_on_pairing_request: user_app_on_pairing_request,
    app_on_tk_exch: user_app_on_tk_exch,
}

#[inline]
fn user_app_on_pairing_request(conidx: u8, param: &da14531_sdk::bindings::gapc_bond_req_ind) {
    rtt_target::rprintln!("pairing request");
    unsafe { da14531_sdk::bindings::default_app_on_pairing_request(conidx, param) }
}

#[inline]
fn user_app_connection(conidx: u8, _param: &GapcConnectionReqInd) {
    let app = unsafe { &mut *APP.get() };

    if app_env_get_conidx(conidx) != GAP_INVALID_CONIDX as u8 {
        // Enable the created profiles/services
        app_prf_enable(conidx);

        // Port this if security is enabled (copied from default handler)
        unsafe { app_easy_security_request(conidx) };

        app.on_connect(Some(conidx));
    } else {
        // No connection has been established
        app.on_connect(None);
    }
    // We want to manage advertise on/off by ourselves. Therefore we do not call
    // `default_app_on_connection`
}

#[inline]
fn user_app_on_set_dev_config_complete() {
    let app = unsafe { &mut *APP.get() };
    rtt_target::rprintln!("user_app_on_set_dev_config_complete");
    app.on_set_dev_config_complete();
    unsafe { default_app_on_set_dev_config_complete() }
}

#[inline]
fn user_app_db_init_complete() {
    let app = unsafe { &mut *APP.get() };
    rtt_target::rprintln!("user_on_db_init_complete");
    app.on_db_init_complete();
}

fn format_passkey(key: &[u8], buf: &mut [u8]) {
    let mut passkey = ((key[0] as u32) << 0)
        | ((key[1] as u32) << 8)
        | ((key[2] as u32) << 16)
        | ((key[3] as u32) << 24);
    for c in buf.iter_mut().rev() {
        *c = (passkey % 10) as u8;
        passkey /= 10;
    }
}

#[inline]
fn user_app_on_tk_exch(conidx: u8, param: &da14531_sdk::bindings::gapc_bond_req_ind) {
    let tk_type = unsafe { param.data.tk_type };
    let tk = param.tk;
    if tk_type == gap_tk_type_GAP_TK_KEY_CONFIRM as u8 {
        let mut buf = [0u8; 6];
        format_passkey(&tk.key, &mut buf);
        rtt_target::rprint!("key: ");
        for c in buf {
            rtt_target::rprint!("{:x}", c);
        }
        rtt_target::rprintln!();
    } else {
        rtt_target::rprintln!("unknown method");
    }
    unsafe { default_app_on_tk_exch(conidx, param) };
}

#[inline]
fn user_app_disconnect(_param: &GapcDisconnectInd) {
    // `default_app_on_disconnect` calls default_operation_adv to start advertising again
    unsafe { default_app_on_disconnect(core::ptr::null()) };
    let app = unsafe { &mut *APP.get() };

    app.on_disconnect();
}

register_app_bond_db_callbacks! {}

// This type uses bitfields which makes it hard to use with bindgen 0.70 (moved to
// user_callback_config.h)
//#[export_name = "user_security_conf"]
//static USER_SECURITY_CONF: da14531_sdk::bindings::security_configuration =
//    da14531_sdk::bindings::security_configuration {
//        _bitfield_align_1: [],
//        _bitfield_1: da14531_sdk::bindings::security_configuration::new_bitfield_1(
//            gap_io_cap_GAP_IO_CAP_DISPLAY_YES_NO,
//            gap_oob_GAP_OOB_AUTH_DATA_NOT_PRESENT,
//        ),
//        auth: gap_auth_mask_GAP_AUTH_BOND as u8,
//        key_size: 0x10,
//        ikey_dist: (gap_kdist_GAP_KDIST_ENCKEY
//            | gap_kdist_GAP_KDIST_IDKEY
//            | gap_kdist_GAP_KDIST_SIGNKEY) as u8,
//        rkey_dist: (gap_kdist_GAP_KDIST_ENCKEY
//            | gap_kdist_GAP_KDIST_IDKEY
//            | gap_kdist_GAP_KDIST_SIGNKEY) as u8,
//        _bitfield_align_2: [],
//        _bitfield_2: da14531_sdk::bindings::security_configuration::new_bitfield_2(
//            gap_sec_req_GAP_SEC2_AUTH_DATA_SGN,
//        ),
//        __bindgen_padding_0: 0,
//    };
