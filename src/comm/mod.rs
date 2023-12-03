pub mod prot;
pub mod udp;
use std::io::{Read, Write};
use std::net::UdpSocket;

pub trait Comm: Sync + Send {
    fn new_server(nonblocking: bool) -> std::io::Result<Self>
    where
        Self: Sized;

    fn wait_client(&mut self) -> std::io::Result<UdpSocket>;

    fn shutdown(&self) -> std::io::Result<()>;
}

pub trait Handle: Send + Read + Write {
    fn shutdown(&mut self) -> std::io::Result<()>;
}

pub fn create_comm(nonblocking: bool) -> std::io::Result<Box<dyn Comm>> {
    Ok(Box::new(udp::UdpComm::new_server(nonblocking)?))
}
