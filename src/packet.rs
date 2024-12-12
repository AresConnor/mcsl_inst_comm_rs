use bitcode::{Decode, Encode};

use crate::payload::*;

#[derive(Encode, Decode, Debug)]
pub struct PacketHeader {
    pub length: u32,
    pub uuid: u128,
}

#[derive(Encode, Decode, Debug)]
pub enum Packet {
    SetUuid(SetUuidPayload),
    StartInst(StartInstPayload),
    ConsoleInput(ConsoleInputPayload),
    KillInst(KillInstPayload),

    LogAppend(LogAppendPayload),
    StatusChange(StatusChangePayload),
    AboutExit(AboutExitPayload),
    Err(ErrPayload),
    Ok(OkPayload),
}

impl Packet {
    pub fn to_message(&self, uuid: u128) -> Vec<u8> {
        let packet_data = bitcode::encode(self);

        let header = PacketHeader {
            length: packet_data.len() as u32,
            uuid,
        };
        let header_data = bitcode::encode(&header);

        let mut data = Vec::with_capacity(packet_data.len() + header_data.len());
        data.extend_from_slice(&header_data);
        data.extend_from_slice(&packet_data);
        data
    }
}
