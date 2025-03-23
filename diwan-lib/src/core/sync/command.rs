use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};

pub trait Message {}

pub enum Command {
    Mes(u8, CommandField, String),
    IncreaseWidth(u8, CommandField, u8),
    DecreaseWidth(u8, CommandField, u8),
    SignalExit(u8),
}

pub enum BrokerCommand {
    Mes(u8, CommandField, String),
    SpawnFrame,
    GetBuffers,
    AddBuffer(BufferedTerminal<UnixTerminal>),
    SignalExit(u8),
}

pub enum CommandField {
    // TODO study and implement other commandfield
    Regular,
    NonRegular,
    SpawnFrame,
}

impl Message for Command {}
impl Message for CommandField {}
impl Message for BrokerCommand {}
