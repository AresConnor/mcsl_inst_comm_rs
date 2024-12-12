use bitcode::{Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum InstProcessStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
}
