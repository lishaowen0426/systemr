use crate::comm::{Comm, Handle};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{Read, Write};
use std::net::{SocketAddr, UdpSocket};

const HOST: &'static str = "127.0.0.1:0";
const ADDR: &'static str = "127.0.0.1:8083";
// for accepting new clients
pub struct UdpComm {
    listener: UdpSocket,
    buffer: [u8; 128], // only for receiving connection
}

pub struct UdpClient {
    pub sock: UdpSocket,
    pub peer: SocketAddr,
}

impl Read for UdpClient {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let (bytes, addr) = self.sock.recv_from(buf)?;
        if self.peer != addr {
            Err(std::io::Error::other(format!(
                "udp: wrong peer addr, expected: {}, actual: {}",
                self.peer, addr,
            )))
        } else {
            Ok(bytes)
        }
    }
}

impl Write for UdpClient {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sock.send_to(buf, self.peer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Handle for UdpClient {
    fn shutdown(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Comm for UdpComm {
    fn new_server(nonblocking: bool) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let listener = UdpSocket::bind(ADDR)?;
        if nonblocking {
            listener.set_nonblocking(nonblocking)?;
        }

        Ok(Self {
            listener,
            buffer: [0; 128],
        })
    }

    fn wait_client(&mut self) -> std::io::Result<UdpSocket> {
        let (_, peer) = self.listener.recv_from(&mut self.buffer)?;

        let sock = UdpSocket::bind(HOST)?;
        let port = sock.local_addr()?.port();

        let mut port_buffer = vec![];
        let _ = port_buffer.write_u16::<NetworkEndian>(port)?;

        //inform peer the new address
        self.listener.send_to(&port_buffer, peer)?;

        Ok(sock)
    }

    fn shutdown(&self) -> std::io::Result<()> {
        todo!()
    }
}
