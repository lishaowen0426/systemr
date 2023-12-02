use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use systemr::{comm, worker};
fn main() -> std::io::Result<()> {
    let sock = comm::create_comm(true)?;
    let stop = Arc::new(AtomicBool::new(false));
    let stop_cloned = stop.clone();
    let stop_cloned2 = stop.clone();

    ctrlc::set_handler(move || {
        stop_cloned.store(true, Ordering::Release);
        return;
    })
    .map_err(|e| match e {
        ctrlc::Error::NoSuchSignal(_) => std::io::Error::other("no such signal"),
        ctrlc::Error::MultipleHandlers => std::io::Error::other("multiple handlers"),
        ctrlc::Error::System(ioe) => ioe,
    })?;

    println!("listen...");

    let mut child = None;

    while !stop.load(Ordering::Acquire) {
        if let Ok(handle) = sock.wait_client() {
            let args = worker::WorkerArgs {
                stop: stop_cloned2,
                handle,
            };

            child = Some(std::thread::spawn(move || worker::run(args)));
            break;
        }
    }

    if let Some(handle) = child {
        let _ = handle.join().expect("join child thread failed");
    }
    Ok(())
}
