use bincode;
use serde::{Deserialize, Serialize};

use crate::payload::*;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
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

type DecodeResult<T> = Result<(T, usize), String>;
pub trait DecodeFromBytes {
    type Target;
    fn from_unchecked_bytes(bytes: &[u8]) -> DecodeResult<Self::Target>;
}

impl DecodeFromBytes for PacketHeader {
    type Target = Self;
    fn from_unchecked_bytes(bytes: &[u8]) -> DecodeResult<Self::Target> {
        let header = bincode::decode_from_slice(bytes, bincode::config::legacy());
        header.map_err(|err| format!("{}", err))
    }
}

impl PacketHeader {
    pub fn from_bytes(bytes: &[u8], uuid: u128) -> DecodeResult<Self> {
        if bytes.len() < PACKET_HEADER_SIZE {
            return Err(format!(
                "PacketHeader: bytes length {} is less than header size {}",
                bytes.len(),
                PACKET_HEADER_SIZE
            ));
        }

        let decoded = Self::from_unchecked_bytes(bytes)?;
        let unchecked_header = &decoded.0;
        let unckecked_uuid = (unchecked_header.uuid1 as u128) << 96
            | (unchecked_header.uuid2 as u128) << 64
            | (unchecked_header.uuid3 as u128) << 32
            | (unchecked_header.uuid4 as u128);
        if unchecked_header.magic != MAGIC || unckecked_uuid != uuid {
            return Err("PacketHeader: invalid header".to_string());
        }
        Ok(decoded)
    }
}

pub const PACKET_BARRIER: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
pub const PACKET_HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();
pub const SYNC_CHUNK_SIZE: usize = 4;
const MAGIC: u16 = 0xA7B8;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub enum Packet {
    Start(StartPayload),
    ConsoleInput(ConsoleInputPayload),
    Kill(KillPayload),

    LogAppend(LogAppendPayload),
    StatusChange(StatusChangePayload),
    AboutExit(AboutExitPayload),
    Err(ErrPayload),
    Ok(OkPayload),
}

impl DecodeFromBytes for Packet {
    type Target = Self;
    fn from_unchecked_bytes(bytes: &[u8]) -> DecodeResult<Self::Target> {
        let header = bincode::decode_from_slice(bytes, bincode::config::standard());
        header.map_err(|err| format!("{}", err))
    }
}

impl Packet {
    pub fn to_bytes(&self, uuid: u128) -> Result<Vec<u8>, String> {
        let packet_data =
            bincode::encode_to_vec(self, bincode::config::standard()).map_err(|e| e.to_string())?;
        let count = if packet_data.len() % SYNC_CHUNK_SIZE == 0 {
            packet_data.len() / SYNC_CHUNK_SIZE
        } else {
            packet_data.len() / SYNC_CHUNK_SIZE + 1
        };

        let header = PacketHeader::new(packet_data.len() as u16, uuid);
        let header_data =
            bincode::encode_to_vec(header, bincode::config::legacy()).map_err(|e| e.to_string())?;

        let mut data =
            Vec::with_capacity(SYNC_CHUNK_SIZE + count * SYNC_CHUNK_SIZE + PACKET_HEADER_SIZE);
        data.extend_from_slice(&PACKET_BARRIER);
        data.extend_from_slice(&header_data);
        data.extend_from_slice(&packet_data);
        Ok(data)
    }
}
