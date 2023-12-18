//! This thread is responsible for polling the RFM transceiver and parsing the data it sends in.
//! It will also take pending messages and send them
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
use barrage::unbounded;
use thiserror::Error;
use crate::io::{Message, MessageTransceiver, TransceiverError, TransceiverResult};

pub struct MessageThread {
    transceiver: MessageTransceiver,
    /// Channel for messages to be transmitted by this thread.
    ingress: (std::sync::mpsc::Sender<Message>, std::sync::mpsc::Receiver<Message>),
    /// Channel for communicating messages received by this thread.
    egress: (barrage::Sender<Message>, barrage::Receiver<Message>),
}

#[derive(Debug, Error)]
enum RunError {
    #[error("{0}")]
    Transceiver(#[from] TransceiverError),
    #[error("Failed to transmit on channel")]
    ChannelTx,
}

impl MessageThread {
    pub fn new() -> TransceiverResult<Self> {
        Ok(Self {
            transceiver: MessageTransceiver::new()?,
            ingress: channel(),
            egress: unbounded(),
        })
    }

    /// Get a sender for sending messages via this thread.
    pub fn get_transmit_handle(&self) -> std::sync::mpsc::Sender<Message> {
        self.ingress.0.clone()
    }

    /// Get a receiver for messages received by this thread.
    pub fn get_receive_handle(&self) -> barrage::Receiver<Message> {
        self.egress.1.clone()
    }

    pub fn start(mut self) -> JoinHandle<()> {
        std::thread::Builder::new()
            .name("message-thread".to_string())
            .spawn(move || {
                loop {
                    match self.run_once() {
                        Ok(_) => {},
                        Err(e) => eprintln!("{e}")
                    }
                }
            })
            .expect("Creating thread")
    }

    fn inform_others_of_message(&self, message: Message) -> Result<(), RunError> {
        self.egress.0.send(message).map_err(|_| RunError::ChannelTx)
    }

    fn get_pending_messages(&self) -> Vec<Message> {
        let mut acc = Vec::new();
        let mut iter = self.ingress.1.try_iter();
        while let Some(msg) = iter.next() {
            acc.push(msg);
        }

        acc
    }

    fn run_once(&mut self) -> Result<(), RunError> {
        // Try receive from the RFM
        if let Some(msg) = self.transceiver.try_receive()? {
            self.inform_others_of_message(msg)?;
        }

        // Send to the RFM
        for pending in self.get_pending_messages() {
            self.transceiver.try_send(pending.into_inner())?;
        }

        Ok(())
    }

}