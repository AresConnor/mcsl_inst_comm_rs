use bitcode::{Decode, Encode};

use crate::encodings::Encoding;

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum InstType {
    Vanilla,
    Forge,
    Fabric,
    Spigot,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum TargetType {
    Jar,
    Script,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
pub struct LaunchConfig {
    pub input_encoding: Encoding,
    pub working_directory: String,
    pub java_args: Vec<String>,
    pub java_path: String,
    pub instance_type: InstType,
    pub target: String,
    pub target_type: TargetType,
}
