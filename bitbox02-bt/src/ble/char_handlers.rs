use da14531_sdk::{
    app_modules::app_env_get_conidx,
    bindings::KE_API_ID_TASK_ID_CUSTS1,
    ble_stack::{
        profiles::{
            custom::custs::custs1::task::{
                Custs1ValWriteInd, Custs1ValueReqInd, KeMsgDynCusts1ValueReqRsp,
            },
            prf::prf_get_task_from_id,
        },
        rwble_hl::error::HlErr::GAP_ERR_NO_ERROR as ATT_ERR_NO_ERROR,
    },
    platform::core_modules::{ke::task::KeTaskId, rwip::TASK_APP},
};
use grounded::uninit::GroundedCell;
use rtt_target::rprintln;

struct CommunicationState {
    message: [u8; 1024],
    offset: usize,
    decoder: u2fframing::Decoder,
    cid: Option<u32>, // If there is a cid, then the host is awaiting an response
}

impl CommunicationState {
    const fn new() -> Self {
        CommunicationState {
            message: [0; 1024],
            offset: 0,
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
    if param.length > 512 {
        /* 512 is max length according to spec */
        rprintln!("write, got more bytes than I can handle");
    }
    let state = unsafe { &mut *STATE.get() };
    if state.cid.is_some() {
        rprintln!("Not ready");
        return;
    }
    let input = unsafe { param.value.as_slice(param.length as usize) };
    match state
        .decoder
        .decode(&mut state.message[state.offset..], input)
    {
        Ok(len) => state.offset += len,
        Err(_) => {
            rprintln!("error");
            state.offset = 0;
            state.decoder = u2fframing::Decoder::new();
        }
    }

    // Read whole message
    if state.offset as u16 == state.decoder.length() {
        rprintln!(
            "cid: {}, cmd: {:x}\nMESSAGE:\n {}",
            state.decoder.cid(),
            state.decoder.cmd(),
            unsafe { core::str::from_utf8_unchecked(&state.message[..state.offset]) },
        );
        state.cid = Some(state.decoder.cid());
        state.decoder = u2fframing::Decoder::new();
    }

    //rprintln!("write, got {:?} bytes", input.len());
}

pub fn data_out_read_handler(param: &Custs1ValueReqInd) {
    const RESPONSE_LEN: u16 = 255;
    let state = unsafe { &mut *STATE.get() };
    rprintln!("read");
    let mut response = KeMsgDynCusts1ValueReqRsp::<RESPONSE_LEN>::new(
        TASK_APP as u16,
        prf_get_task_from_id(KE_API_ID_TASK_ID_CUSTS1 as KeTaskId),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    match state.cid {
        Some(cid) => {
            let echo_len = 11.min(state.offset);
            let mut response_msg = [0u8; 13];
            response_msg[0..2].copy_from_slice(b"> ");
            response_msg[2..2 + echo_len].copy_from_slice(&state.message[..echo_len]);
            response.fields().length = (2 + echo_len + 7) as u16;

            let value = unsafe { response.fields().value.as_mut_slice(response_msg.len() + 7) };

            let mut encoder = u2fframing::Encoder::new(cid);
            let len = encoder
                .start(value, &response_msg, response_msg.len() as u16, 0x81)
                .unwrap();

            if len != response_msg.len() {
                panic!("Could not send whole message in one packet")
            }
            state.cid = None;
            state.offset = 0;
            state.message.fill(0);
        }
        None => response.fields().length = 0,
    }

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    response.send();
}
