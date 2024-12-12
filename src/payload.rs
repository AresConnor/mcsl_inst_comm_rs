use crate::inst_status::InstProcessStatus;
use crate::launch_config::LaunchConfig;
use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct SetUuidPayload {}

#[derive(Encode, Decode, Debug)]
pub struct StartInstPayload {
    pub config: LaunchConfig,
}
#[derive(Encode, Decode, Debug)]
pub struct ConsoleInputPayload {
    pub input: Vec<u8>,
}

#[derive(Encode, Decode, Debug)]
pub struct KillInstPayload {}

#[derive(Encode, Decode, Debug)]
pub struct LogAppendPayload {
    pub log: Vec<u8>,
}

#[derive(Encode, Decode, Debug)]
pub struct StatusChangePayload {
    pub status: InstProcessStatus,
}

#[derive(Encode, Decode, Debug)]
pub struct AboutExitPayload {
    pub exit_code: i32,
}

#[derive(Encode, Decode, Debug)]
pub struct ErrPayload {}

#[derive(Encode, Decode, Debug)]
pub struct OkPayload {}
