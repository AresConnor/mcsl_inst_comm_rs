use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, bincode::Encode, bincode::Decode, PartialEq, Eq, Copy, Clone,Default)]
pub enum InstProcessStatus {
    Starting,
    Running,
    Stopping,
    #[default]
    Stopped,
    Crashed,
}
