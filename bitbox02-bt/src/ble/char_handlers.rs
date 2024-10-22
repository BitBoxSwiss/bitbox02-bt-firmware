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
};
use grounded::uninit::GroundedCell;
use rtt_target::rprintln;
use u2fframing::NeedMore;

struct CommunicationState {
    message: Vec<u8>,
    decoder: u2fframing::Decoder,
    cid: Option<u32>, // If there is a cid, then the host is awaiting an response
}

impl CommunicationState {
    const fn new() -> Self {
        CommunicationState {
            message: Vec::new(),
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
    if param.length > 64 {
        rprintln!("write, got more bytes than expected");
    }
    let state = unsafe { &mut *STATE.get() };
    if state.cid.is_some() {
        rprintln!("Not ready");
        return;
    }
    let input = unsafe { param.value.as_slice(param.length as usize) };

    if state.message.is_empty() {
        // Figure out size of allocation
        let Err(u2fframing::error::Error::BufferSize(len)) =
            state.decoder.decode(&mut [0u8; 0], input)
        else {
            panic!();
        };

        state.message.resize(len.into(), 0);
    }

    match state.decoder.decode(&mut state.message[..], input) {
        Ok(NeedMore::Done(len)) => {
            // Read whole message
            rprintln!(
                "cid: {}, cmd: {:x}\nMESSAGE ({} bytes):\n {}",
                state.decoder.cid(),
                state.decoder.cmd(),
                len,
                unsafe { core::str::from_utf8_unchecked(&state.message[..len.into()]) },
            );
            state.cid = Some(state.decoder.cid());
            state.decoder = u2fframing::Decoder::new();
        }
        Ok(_) => {}
        Err(_) => {
            rprintln!("error");
            state.decoder = u2fframing::Decoder::new();
        }
    }
}

pub fn data_out_read_handler(param: &Custs1ValueReqInd) {
    const RESPONSE_LEN: u16 = 64;
    let state = unsafe { &mut *STATE.get() };
    rprintln!("read");
    let mut response = KeMsgDynCusts1ValueReqRsp::<RESPONSE_LEN>::new(
        TASK_APP as u16,
        prf_get_task_from_id(TASK_ID_CUSTS1 as u16),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    match state.cid {
        Some(cid) => {
            let echo_len = (RESPONSE_LEN as usize - 2 - 7).min(state.message.len());
            let mut response_msg = [0u8; RESPONSE_LEN as usize];
            response_msg[0..2].copy_from_slice(b"> ");
            response_msg[2..2 + echo_len].copy_from_slice(&state.message[..echo_len]);
            //response.fields().length = (2 + echo_len + 7) as u16;
            response.fields().length = RESPONSE_LEN;
            //response.fields().length = 1;

            let value = unsafe { response.fields().value.as_mut_slice(RESPONSE_LEN.into()) };

            let mut encoder = u2fframing::Encoder::new(cid);
            let len = encoder
                .start(value, &response_msg[0..2 + echo_len], 0x81)
                .unwrap();

            if len != 2 + echo_len {
                panic!("Could not send whole message in one packet")
            }
            state.cid = None;
            state.message.clear();
        }
        None => response.fields().length = 0,
    }

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    rprintln!("Send {} message back", response.fields().length);

    response.send();
}
