use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, bincode::Encode, bincode::Decode, PartialEq, Eq)]
pub enum InstProcessStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
}
