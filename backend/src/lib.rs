#![feature(let_chains)]

use std::cell::OnceCell;
use std::thread::sleep;
use std::time::Duration;
use crate::io::TransceiverError;
use crate::thread::MessageThread;

mod bridge;
mod bridge_generated;
mod io;
mod thread;

pub fn start() -> Result<(), TransceiverError> {
    let message_thread = MessageThread::new()?;
    let _ = message_thread.start();

    loop {
        sleep(Duration::from_millis(500));
    }

    Ok(())
}
