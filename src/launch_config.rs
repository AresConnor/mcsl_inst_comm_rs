use serde::{Deserialize, Serialize};

use crate::encodings::Encoding;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub enum InstType {
    Vanilla,
    Forge,
    Fabric,
    Spigot,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub enum TargetType {
    Jar,
    Script,
}

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug, PartialEq, Eq)]
pub struct LaunchConfig {
    pub input_encoding: Encoding,
    pub working_directory: String,
    pub java_args: Vec<String>,
    pub java_path: String,
    pub instance_type: InstType,
    pub target: String,
    pub target_type: TargetType,
}
