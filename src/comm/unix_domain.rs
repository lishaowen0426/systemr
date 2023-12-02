use crate::comm::{Comm, Handle};
use std::fs;
use std::io;
use std::os::unix::net::{UnixListener, UnixStream};

const SOCK_PATH: &str = "/tmp/systemr_sock";

pub struct UnixComm {
    listener: UnixListener,
}

pub struct UnixSocket {
    sock: UnixStream,
}

impl io::Read for UnixSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.sock.read(buf)
    }
}

impl io::Write for UnixSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sock.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Handle for UnixSocket {
    fn recv(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        io::Read::read(self, buf)
    }

    fn send(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        io::Write::write(self, buf)
    }

    fn shutdown(&mut self) -> std::io::Result<()> {
        self.sock.shutdown(std::net::Shutdown::Both)
    }
}

impl Comm for UnixComm {
    fn new_server(nonblocking: bool) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        if let Ok(_) = fs::metadata(SOCK_PATH) {
            fs::remove_file(SOCK_PATH)?;
        }

        let listener = UnixListener::bind(SOCK_PATH)?;

        listener.set_nonblocking(nonblocking)?;
        Ok(UnixComm { listener })
    }

    fn wait_client(&self) -> std::io::Result<Box<dyn Handle>> {
        let (sock, _) = self.listener.accept()?;
        return Ok(Box::new(UnixSocket { sock }));
    }

    fn shutdown(&self) -> std::io::Result<()> {
        if let Ok(_) = fs::metadata(SOCK_PATH) {
            fs::remove_file(SOCK_PATH)?;
        }
        Ok(())
    }
}

impl Drop for UnixComm {
    fn drop(&mut self) {
        self.shutdown().ok();
    }
}
