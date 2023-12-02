use crate::comm::{prot, Handle};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct WorkerArgs {
    pub stop: Arc<AtomicBool>,
    pub handle: Box<dyn Handle>,
}

pub fn run(mut args: WorkerArgs) -> std::io::Result<()> {
    let mut buf: [u8; 4096] = [0; 4096];
    loop {
        if args.stop.load(Ordering::Acquire) {
            match args.handle.shutdown() {
                Ok(_) => break,
                Err(e) => return std::io::Result::Err(e),
            }
        }
        let read = args.handle.read(&mut buf)?;
        if read == 0 {
            break;
        } else {
            println!("bytes read: {}", read);
            let p = prot::Message::try_from(&buf[..read]).map_err(|_: serde_json::Error| {
                std::io::Error::other("from bytes to prot failed")
            })?;

            println!("p: {:?}", p);
        }
    }
    println!("child exited...\n");

    std::io::Result::Ok(())
}
