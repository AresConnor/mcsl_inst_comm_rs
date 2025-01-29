use std::{ffi::OsStr, future::Future, path::Path};

pub mod inst_status;
pub mod packet;
pub mod payload;

pub type CommResult = Result<(), ()>;

pub trait InstCommRpc {
    fn start<P, I, M, S>(&self, program: P, args: I, env: M) -> impl Future<Output = CommResult>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = S>,
        M: IntoIterator<Item = (S, S)>,
        S: AsRef<OsStr>;
    fn write_console(&self, text: Vec<u8>) -> impl Future<Output = CommResult>;
    fn kill(&self) -> impl Future<Output = CommResult>;
}
