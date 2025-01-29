use std::collections::HashMap;

use crate::inst_status::InstProcessStatus;
use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct StartPayload {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct ConsoleInputPayload {
    pub input: Vec<u8>,
}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct KillPayload {}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct LogAppendPayload {
    pub log: Vec<u8>,
}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct StatusChangePayload {
    pub status: InstProcessStatus,
}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct AboutExitPayload {
    pub exit_code: i32,
}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct ErrPayload {}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct OkPayload {}
