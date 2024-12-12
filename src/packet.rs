use bitcode::{Decode, Encode};

use crate::payload::*;

#[derive(Encode, Decode, Debug)]
pub struct PacketHeader {
    pub length: u16,
    pub magic: u16,
    pub uuid1: u32,
    pub uuid2: u32,
    pub uuid3: u32,
    pub uuid4: u32,
}

impl PacketHeader {
    pub fn new(length: u16, uuid: u128) -> Self {
        let uuid1 = (uuid >> 96) as u32;
        let uuid2 = (uuid >> 64) as u32;
        let uuid3 = (uuid >> 32) as u32;
        let uuid4 = uuid as u32;

        Self {
            length,
            magic: MAGIC,
            uuid1,
            uuid2,
            uuid3,
            uuid4,
        }
    }
}

type DecodeResult<T> = Result<T, String>;
pub trait DecodeFromBytes {
    type Target;
    fn from_bytes(bytes: &[u8]) -> DecodeResult<Self::Target>;
}

impl DecodeFromBytes for PacketHeader {
    type Target = Self;
    fn from_bytes(bytes: &[u8]) -> DecodeResult<Self::Target> {
        let header = bitcode::decode::<PacketHeader>(bytes);
        header.map_err(|err| format!("{}", err))
    }
}

const PACKET_BARRIER: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
const PACKET_HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();
const MAGIC: u16 = 0xDEAD;
const CHUNK_SIZE: usize = 4;

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

impl DecodeFromBytes for Packet {
    type Target = Self;
    fn from_bytes(bytes: &[u8]) -> DecodeResult<Self::Target> {
        let header = bitcode::decode::<Self>(bytes);
        header.map_err(|err| format!("{}", err))
    }
}

impl Packet {
    pub fn to_message(&self, uuid: u128) -> Vec<u8> {
        let packet_data = bitcode::encode(self);
        let length = if packet_data.len() % CHUNK_SIZE == 0 {
            packet_data.len() / CHUNK_SIZE
        } else {
            packet_data.len() / CHUNK_SIZE + 1
        };

        let header = PacketHeader::new(length as u16, uuid);
        let header_data = bitcode::encode(&header);

        let mut data = Vec::with_capacity(CHUNK_SIZE + length * CHUNK_SIZE + PACKET_HEADER_SIZE);
        data.extend_from_slice(&PACKET_BARRIER);
        data.extend_from_slice(&header_data);
        data.extend_from_slice(&packet_data);
        data
    }
}
