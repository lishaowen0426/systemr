use crate::comm::udp::UdpClient;
use crate::comm::{prot, Handle};
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

const ACK: &'static [u8] = b"ack";

pub struct WorkerArgs {
    pub stop: Arc<AtomicBool>,
    pub sock: UdpSocket,
}

pub fn run(args: WorkerArgs) -> std::io::Result<()> {
    //wait acknowledgement
    //
    // set acknowledgement timeout = 10s
    let _ = args.sock.set_read_timeout(Some(Duration::from_secs(5)));
    let mut client = unsafe {
        let mut ack: [u8; 128] = [0; 128];

        let (cnt, peer) = args.sock.recv_from(&mut ack)?;
        if cnt != ACK.len() {
            return Err(std::io::Error::other(format!(
                "wrong ack size, expected {}, actual {}",
                ACK.len(),
                cnt
            )));
        }

        if &ack[..ACK.len()] != ACK {
            return Err(std::io::Error::other(format!(
                "wrong ack content, expected {}, actual {}",
                std::str::from_utf8_unchecked(ACK),
                std::str::from_utf8_unchecked(&ack[..cnt]),
            )));
        }
        UdpClient {
            sock: args.sock,
            peer,
        }
    };
    let _ = client.sock.set_read_timeout(None);

    let mut buf: [u8; 4096] = [0; 4096];
    loop {
        if args.stop.load(Ordering::Acquire) {
            match client.shutdown() {
                Ok(_) => break,
                Err(e) => return std::io::Result::Err(e),
            }
        }
        let read = client.read(&mut buf)?;

        if read == 0 {
            break;
        } else {
            println!("{}", std::str::from_utf8(&buf[..read]).unwrap());
            let p = prot::Message::try_from(&buf[..read]).map_err(|e: serde_json::Error| {
                println!("err: {}", e);
                std::io::Error::other("from bytes to prot failed")
            })?;

            println!("p: {:?}", p);
            if let prot::Message::Disconnected = p {
                println!("child exited...\n");
                break;
            }
            buf.fill(0);
        }
    }
    std::io::Result::Ok(())
}
