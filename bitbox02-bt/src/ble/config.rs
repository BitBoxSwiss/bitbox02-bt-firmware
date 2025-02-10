use da14531_sdk::{
    app_modules::{
        app_cfg_addr_src,
        app_custs::{custs1::app_custs1_create_db, CustPrfFuncCallbacks},
        configure_custom_server1_service, default_handlers_configuration, ms_to_ble_slots,
        ms_to_timer_units, AdvertiseConfiguration, APP_CFG_ADDR_STATIC, DEF_ADV_WITH_TIMEOUT,
        DEF_SEC_REQ_NEVER,
    },
    ble_stack::host::gap::GAP_GEN_DISCOVERABLE,
    platform::core_modules::common::{ADV_ALLOW_SCAN_ANY_CON_ANY, ADV_ALL_CHNLS_EN},
};
use rtt_target::rprintln;

const PAYLOAD_LENGTH: u16 = 512;

// Setup service database
configure_custom_server1_service![
    svc1: {
        uuid: 0xE1511A45_F3DB_44C0_82B8_6C880790D1F1_u128,
        characteristics: {
            rx: {
                uuid: 0x0001,
                permissions: (WRITE_ENABLED | WRITE_REQUEST_ACCEPTED | WRITE_COMMAND_ACCEPTED),
                length: PAYLOAD_LENGTH,
                user_description: "RX",
                write_handler: crate::ble::char_handlers::rx_write_handler,
            },
            tx: {
                uuid: 0x0002,
                permissions: (INDICATION_AUTH),
                length: PAYLOAD_LENGTH,
                user_description: "TX",
                read_handler: crate::ble::char_handlers::tx_read_handler,
            },
            product: {
                uuid: 0x0003,
                permissions: (READ_ENABLED | INDICATION_ENABLED),
                length: 20,
                user_description: "Product",
                read_handler: crate::ble::char_handlers::product_read_handler,
                write_handler: crate::ble::char_handlers::product_write_handler,
            }
        },
    }
];

// Set the advertisement period
const ADV_PERIOD: i32 = ms_to_timer_units(4000) as i32;

// Configure default handlers
default_handlers_configuration! {
    adv_scenario: DEF_ADV_WITH_TIMEOUT,
    advertise_period: ADV_PERIOD,
    security_request_scenario: DEF_SEC_REQ_NEVER
}

// Define user-specific advertisement configuration
#[no_mangle]
pub static USER_ADV_CONF: AdvertiseConfiguration = AdvertiseConfiguration {
    addr_src: app_cfg_addr_src(APP_CFG_ADDR_STATIC),
    intv_min: ms_to_ble_slots(100),
    intv_max: ms_to_ble_slots(150),
    channel_map: ADV_ALL_CHNLS_EN as u8,
    mode: GAP_GEN_DISCOVERABLE as u8,
    adv_filt_policy: ADV_ALLOW_SCAN_ANY_CON_ANY as u8,
    peer_addr: [0x1, 0x2, 0x3, 0x4, 0x5, 0x6],
    peer_addr_type: 0,
};
