use super::command::Command;
use super::command::Message;
use tokio::sync::mpsc;

pub struct Actor {
    mailbox: mpsc::Receiver<Command>,
    sender: mpsc::Sender<Command>,
}
