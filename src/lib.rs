use std::future::Future;

use launch_config::LaunchConfig;

pub mod encodings;
pub mod inst_status;
pub mod launch_config;
pub mod packet;
pub mod payload;

pub type CommResult = Result<(), ()>;

pub trait InstCommRpc {
    fn start(&self, config: LaunchConfig) -> impl Future<Output = CommResult>;
    fn write_console(&self, text: Vec<u8>) -> impl Future<Output = CommResult>;
    fn kill(&self) -> impl Future<Output = CommResult>;
}
