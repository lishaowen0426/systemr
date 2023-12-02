pub mod prot;
mod unix_domain;
use std::io::{Read, Write};

pub trait Comm: Sync + Send {
    fn new_server(nonblocking: bool) -> std::io::Result<Self>
    where
        Self: Sized;

    fn wait_client(&self) -> std::io::Result<Box<dyn Handle>>;

    fn shutdown(&self) -> std::io::Result<()>;
}

pub trait Handle: Send + Read + Write {
    fn recv(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn send(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn shutdown(&mut self) -> std::io::Result<()>;
}

pub fn create_comm(nonblocking: bool) -> std::io::Result<Box<dyn Comm>> {
    Ok(Box::new(unix_domain::UnixComm::new_server(nonblocking)?))
}
