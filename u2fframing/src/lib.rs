// Copyright 2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//! U2FHID protocol specification ch. Message- and packet structure:
//!
//! "With this approach, a message with a payload less or equal to (s - 7) may be sent as one packet.
//! A larger message is then divided into one or more continuation packets, starting with sequence
//! number 0, which then increments by one to a maximum of 127. With a packet size of 64 bytes (max
//! for full-speed devices), this means that the maximum message payload length is 64 - 7 + 128 *
//! (64 - 5) = 7609 bytes."
//!
//! This implementation supports varying packet sizes.
#![no_std]

use byteorder::{BigEndian, ByteOrder};
use core::result;

pub const HEADER_INIT_LEN: usize = 7;
pub const HEADER_CONT_LEN: usize = 5;

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        /// Buffer must fit the whole message (u16 is the required len)
        BufferSize(u16),
        /// Packet size must be 7 bytes for init packet and 5 for continuation packets
        PacketSize,
        /// CMD must have highest bit set
        InvalidCmd,
        /// Packets must have the same CID
        NotSameCid,
        /// Packets must come in order
        UnexpectedSeq,
        /// It is not possible to send more than 127 continuation packets
        TooManyPackets,
        /// Init packet must come before continuation
        ExpectingInitPacket,
    }
}

use error::Error;

type Result<T> = result::Result<T, Error>;

pub struct Encoder {
    /// Channel Identifier
    cid: u32,
    /// Packet Seuqence
    seq: u8,
    /// Message length
    len: u16,
    /// Bytes written
    written: u16,
}

impl Encoder {
    pub const fn new(cid: u32) -> Self {
        Encoder {
            cid,
            seq: 0,
            len: 0,
            written: 0,
        }
    }

    /// Needs 7 extra bytes in buf for first header.
    /// `message` length must fit in u16
    /// Returns bytes read from message
    pub fn start(&mut self, mut packet: &mut [u8], message: &[u8], cmd: u8) -> Result<usize> {
        // Buffer must be large enough to fit header
        if packet.len() < HEADER_INIT_LEN {
            return Err(Error::PacketSize);
        }

        BigEndian::write_u32(packet, self.cid);

        // Bit 7 must always be set
        if (cmd & 0x80) != 0x80 {
            return Err(Error::InvalidCmd);
        }
        packet[4] = cmd;

        // Length is encoded using big endianess
        BigEndian::write_u16(&mut packet[5..], message.len().try_into().unwrap());
        self.len = message.len().try_into().unwrap();

        // Skip forward, over the header
        packet = &mut packet[HEADER_INIT_LEN..];

        let end = packet.len().min(message.len());
        packet[..end].copy_from_slice(&message[..end]);
        self.written = end as u16;
        Ok(end)
    }

    /// Needs 5 extra bytes in buf for continuation header.
    /// Returns bytes read from message
    pub fn continuation(&mut self, mut buf: &mut [u8], message: &[u8]) -> Result<usize> {
        if buf.len() < HEADER_CONT_LEN {
            return Err(Error::PacketSize);
        }
        // Write header of rest of packets
        BigEndian::write_u32(buf, self.cid);
        if (self.seq & 0x80) == 0x80 {
            return Err(Error::TooManyPackets);
        }
        buf[4] = self.seq;
        self.seq += 1;

        // Skip over the header
        buf = &mut buf[HEADER_CONT_LEN..];

        let end = buf
            .len()
            .min(message.len())
            .min((self.len - self.written) as usize);
        buf[..end].copy_from_slice(&message[..end]);
        self.written += end as u16;
        Ok(end)
    }
}

pub struct Decoder {
    state: DecoderState,
}

enum DecoderState {
    Pending,
    Reading {
        cid: u32,
        len: u16,
        read: u16,
        cmd: u8,
        seq: u8,
    },
    Done {
        cid: u32,
        cmd: u8,
    },
}

#[derive(Debug)]
pub enum NeedMore {
    /// Returns number of bytes read
    NeedMore(u16),
    /// Returns number of bytes read
    Done(u16),
}

impl Decoder {
    pub const fn new() -> Self {
        Decoder {
            state: DecoderState::Pending,
        }
    }

    /// Decode function. Will fail in case CID doesn't match in every `buf`. Returns bytes read.
    pub fn decode(&mut self, message: &mut [u8], packet: &[u8]) -> Result<NeedMore> {
        if packet.len() < HEADER_CONT_LEN {
            // We can't even check what kind of packet it is
            return Err(Error::PacketSize);
        }
        // Check if packet is init or continuation, the cmd/seq byte's msb decides
        if (packet[4] & 0x80) == 0x80 {
            // init packet
            if packet.len() < HEADER_INIT_LEN {
                // It could potentially be a continuation packet
                return Err(Error::PacketSize);
            }
            let cid = BigEndian::read_u32(&packet[..4]);
            let cmd = packet[4];
            let len = BigEndian::read_u16(&packet[5..7]);
            if message.len() < len.into() {
                // The `message` buffer needs to fit the whole message. Try again with a larger
                // buffer
                return Err(Error::BufferSize(len));
            }

            let end = message.len().min(packet.len() - 7).min(len as usize);
            message[..end].copy_from_slice(&packet[7..7 + end]);
            if (end as u16) < len {
                self.state = DecoderState::Reading {
                    cid,
                    len,
                    read: end as u16,
                    seq: 0,
                    cmd,
                };
                Ok(NeedMore::NeedMore(end as u16))
            } else {
                self.state = DecoderState::Done { cid, cmd };
                Ok(NeedMore::Done(len))
            }
        } else {
            // continuation packet
            match self.state {
                DecoderState::Pending => Err(Error::ExpectingInitPacket),
                DecoderState::Done { .. } => Err(Error::ExpectingInitPacket),
                DecoderState::Reading {
                    cid,
                    len,
                    read,
                    seq,
                    cmd,
                } => {
                    let read_cid = BigEndian::read_u32(&packet[..4]);
                    if cid != read_cid {
                        return Err(Error::NotSameCid);
                    }
                    let read_seq = packet[4];
                    if seq != read_seq {
                        return Err(Error::UnexpectedSeq);
                    }
                    let start = read as usize;
                    let end = (packet.len() - 5).min((len - read) as usize);
                    message[start..start + end].copy_from_slice(&packet[5..5 + end]);
                    if start + end == len.into() {
                        self.state = DecoderState::Done { cid, cmd };
                        Ok(NeedMore::Done(len))
                    } else {
                        self.state = DecoderState::Reading {
                            cid,
                            len,
                            read: read + end as u16,
                            seq: seq + 1,
                            cmd,
                        };
                        Ok(NeedMore::NeedMore(read + end as u16))
                    }
                }
            }
        }
    }

    pub fn cmd(&self) -> u8 {
        match self.state {
            DecoderState::Pending => panic!(),
            DecoderState::Reading { cmd, .. } => cmd,
            DecoderState::Done { cmd, .. } => cmd,
        }
    }

    pub fn cid(&self) -> u32 {
        match self.state {
            DecoderState::Pending => panic!(),
            DecoderState::Reading { cid, .. } => cid,
            DecoderState::Done { cid, .. } => cid,
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Decoder::new()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::*;
    use std::vec::Vec;

    #[test]
    fn test_encode_single() {
        let cid = 0x12345678;
        let cmd = 0x80;
        let mut msg = [0u8; 64];
        msg[..4].copy_from_slice(b"\x01\x02\x03\x04");
        let mut codec = Encoder::new(cid);
        let mut buf = [0u8; 100];
        let len = codec.start(&mut buf[..], &msg, 4, cmd).unwrap();
        assert_eq!(len, 4);
        let expect = b"\x12\x34\x56\x78\x80\x00\x04\x01\x02\x03\x04";
        assert_eq!(&buf[..11], &expect[..]);
    }

    #[test]
    fn test_encode_multi() {
        let cid = 0x12345678;
        let cmd = 0x81;
        let mut msg: &[u8] = &(0..65u8).collect::<Vec<u8>>();
        let mut codec = Encoder::new(cid);
        let mut buf = [0u8; 21];
        let mut expect = [0u8; 21];

        let len = codec.start(&mut buf, msg, msg.len() as u16, cmd).unwrap();
        expect[..7].copy_from_slice(b"\x12\x34\x56\x78\x81\x00\x41");
        expect[7..].copy_from_slice(&msg[..len]);
        assert_eq!(len, 14);
        assert_eq!(buf, expect);

        // skip forward the sent bytes
        msg = &msg[len..];

        let mut seq: u8 = 0;
        loop {
            let len = codec.continuation(&mut buf, msg).unwrap();
            expect[..4].copy_from_slice(b"\x12\x34\x56\x78");
            expect[4] = seq;
            expect[5..5 + len].copy_from_slice(&msg[..len]);
            seq += 1;
            assert_eq!(&buf[..], &expect[..]);
            if len == msg.len() {
                // We have sent all the bytes
                break;
            }
            // skip forward the sent bytes
            msg = &msg[len..];
        }
    }

    #[test]
    fn test_encode_too_small_buffer() {
        let cid = 0x12345678;
        let cmd = 0x55;
        let msg = b"\x01\x02\x03\x04";
        let mut codec = Encoder::new(cid);
        let mut buf = [0u8; 4];
        let Err(Error::PacketSize) = codec.start(&mut buf[..], msg, msg.len() as u16, cmd) else {
            panic!("Should've returned InvalidBufferSize");
        };
    }

    #[test]
    fn test_encode_invalid_cmd() {
        let cid = 0x12345678;
        let cmd = 0x00;
        let msg = b"\x01\x02\x03\x04";
        let mut codec = Encoder::new(cid);
        let mut buf = [0u8; 64];
        let Err(Error::InvalidCmd) = codec.start(&mut buf[..], msg, msg.len() as u16, cmd) else {
            panic!("Should've returned InvalidCmd");
        };
    }

    #[test]
    fn test_decode_single() {
        let mut decoder = Decoder::default();
        let mut packet = [0u8; 64];
        let mut msg = [0u8; 64];
        packet[..11].copy_from_slice(b"\xEE\xEE\xEE\xEE\xFF\x00\x04\x01\x02\x03\x04");

        match decoder.decode(&mut msg[..], &packet[..]) {
            Ok(NeedMore::Done(len)) => {
                assert_eq!(len, 4);

                assert_eq!(decoder.cid(), 0xEEEEEEEE);
                assert_eq!(decoder.cmd(), 0xFF);
                assert_eq!(&msg[..len as usize], b"\x01\x02\x03\x04");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_decode_multi() {
        let payload: Vec<u8> = (0..150u8).collect();
        let mut decoder = Decoder::default();
        let mut packet1 = [0u8; 64];
        let mut packet2 = [0u8; 64];
        let mut packet3 = [0u8; 64];
        packet1[..7].copy_from_slice(b"\xEE\xEE\xEE\xEE\x91\x00\x96");
        packet1[7..64].copy_from_slice(&payload[..57]);
        packet2[..5].copy_from_slice(b"\xEE\xEE\xEE\xEE\x00");
        packet2[5..64].copy_from_slice(&payload[57..57 + 59]);
        packet3[..5].copy_from_slice(b"\xEE\xEE\xEE\xEE\x01");
        packet3[5..39].copy_from_slice(&payload[57 + 59..57 + 59 + 34]);

        let mut data = [0u8; 200];
        match decoder.decode(&mut data[..], &packet1) {
            Ok(NeedMore::NeedMore(len)) => {
                assert_eq!(len, 57);

                assert_eq!(decoder.cid(), 0xEEEEEEEE);
                assert_eq!(decoder.cmd(), 0x91);
            }
            _ => panic!(),
        }

        match decoder.decode(&mut data[..], &packet2) {
            Ok(NeedMore::NeedMore(len)) => {
                assert_eq!(len, 57 + 59);

                assert_eq!(decoder.cid(), 0xEEEEEEEE);
                assert_eq!(decoder.cmd(), 0x91);
            }
            unexpected => panic!("{:?}", unexpected),
        }

        match decoder.decode(&mut data[..], &packet3) {
            Ok(NeedMore::Done(len)) => {
                assert_eq!(len, 150);
                assert_eq!(&data[..len as usize], &payload[..]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_decode_wrong_seq() {
        let payload: Vec<u8> = (0..65u8).collect();
        let mut decoder = Decoder::default();
        let mut packet1 = [0u8; 64];
        let mut packet2 = [0u8; 64];
        packet1[..7].copy_from_slice(b"\xEE\xEE\xEE\xEE\xA0\x00\x41");
        packet1[7..64].copy_from_slice(&payload[..57]);
        packet2[..5].copy_from_slice(b"\xEE\xEE\xEE\xEE\x01");
        packet2[5..13].copy_from_slice(&payload[57..]);

        let mut data = [0u8; 100];
        let _ = decoder.decode(&mut data[..], &packet1).unwrap();
        let Err(Error::UnexpectedSeq) = decoder.decode(&mut data[..], &packet2) else {
            panic!("Should've returned UnexpectedSeq");
        };
    }

    #[test]
    fn test_decode_wrong_cid() {
        let payload: Vec<u8> = (0..65u8).collect();
        let mut decoder = Decoder::default();
        let mut packet1 = [0u8; 64];
        let mut packet2 = [0u8; 64];
        packet1[..7].copy_from_slice(b"\xEE\xEE\xEE\xEE\x80\x00\x41");
        packet1[7..64].copy_from_slice(&payload[..57]);
        packet2[..5].copy_from_slice(b"\xEE\xEE\xEE\xE1\x01");
        packet2[5..13].copy_from_slice(&payload[57..]);

        let mut data = [0u8; 100];
        let _ = decoder.decode(&mut data[..], &packet1).unwrap();
        let Err(Error::NotSameCid) = decoder.decode(&mut data[..], &packet2) else {
            panic!("Should've returned NotSameCid");
        };
    }

    #[test]
    fn test_decode_wrong_cmd() {
        let mut decoder = Decoder::default();
        let mut packet = [0u8; 64];
        let mut msg = [0u8; 64];
        packet[..11].copy_from_slice(b"\xEE\xEE\xEE\xEE\x00\x00\x04\x01\x02\x03\x04");

        let Err(Error::ExpectingInitPacket) = decoder.decode(&mut msg[..], &packet) else {
            panic!("Should've returned ExpectingInitPacket");
        };
    }
}
