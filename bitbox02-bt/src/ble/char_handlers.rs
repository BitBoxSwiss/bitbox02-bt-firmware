use alloc::vec::Vec;
use da14531_sdk::{
    app_modules::app_env_get_conidx,
    ble_stack::{
        profiles::custom::custs::custs1::task::{
            Custs1ValWriteInd, Custs1ValueReqInd, KeMsgDynCusts1ValueReqRsp,
        },
        profiles::prf::prf_get_task_from_id,
        rwble_hl::error::HlErr::GAP_ERR_NO_ERROR as ATT_ERR_NO_ERROR,
    },
    platform::core_modules::rwip::{KeApiId::TASK_ID_CUSTS1, KeTaskType::TASK_APP},
    platform::driver::uart,
};
use grounded::uninit::GroundedCell;
use rtt_target::rprintln;
use u2fframing::NeedMore;

struct CommunicationState {
    message: [u8; 64],
    decoder: u2fframing::Decoder,
    cid: Option<u32>, // If there is a cid, then the host is awaiting an response
}

impl CommunicationState {
    const fn new() -> Self {
        CommunicationState {
            message: [0u8; 64],
            decoder: u2fframing::Decoder::new(),
            cid: None,
        }
    }
}

impl grounded::const_init::ConstInit for CommunicationState {
    const VAL: Self = Self::new();
}

static STATE: GroundedCell<CommunicationState> = GroundedCell::const_init();

pub fn data_in_write_handler(param: &Custs1ValWriteInd) {
    rprintln!("data_in_write_handler, length: {}", param.length);
    if param.length > 64 {
        rprintln!("write, got more bytes than expected");
        return;
    }
    let input = unsafe { param.value.as_slice(param.length as usize) };
    rprintln!("UART: writing ({})", hex::encode(input));
    uart::send_blocking(input);
}

pub fn data_out_read_handler(param: &Custs1ValueReqInd) {
    const RESPONSE_LEN: u16 = 64;
    let mut response = KeMsgDynCusts1ValueReqRsp::<RESPONSE_LEN>::new(
        TASK_APP as u16,
        prf_get_task_from_id(TASK_ID_CUSTS1 as u16),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    // Value field
    let value = unsafe { response.fields().value.as_mut_slice(RESPONSE_LEN.into()) };

    //let mut buf: [u8; 64] = [0; 64];
    uart::recv_blocking(&mut value[..]);
    rprintln!("UART: read (data: {})", hex::encode(value));
    response.fields().length = 64;

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    rprintln!("BLE: Send message (len: {})", response.fields().length);

    response.send();
}
