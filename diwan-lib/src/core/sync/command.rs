use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};

use crate::screen::DWidget;

pub trait Message {}

pub enum Command {
    Mes(u8, CommandField, String),
    IncreaseWidth(u8, CommandField, u8),
    DecreaseWidth(u8, CommandField, u8),
    SignalExit(u8),
}

pub enum BrokerCommand<'a> {
    Mes(u8, CommandField, String),
    // SpawnBuffer(u8),// ✅ the u8 field is for the id of the adequate frame ( to identify who send what !  )
    GetBuffers(u8),                                               // ✅
    AddBuffer(u8, (&'a BufferedTerminal<UnixTerminal>, DWidget)), // ✅
    SignalExit(u8),
    Ok(u8),                                                            // ✅
    OkBuffers(Vec<(u8, &'a BufferedTerminal<UnixTerminal>, DWidget)>), // can i do that ?
}

pub enum CommandField {
    // TODO study and implement other commandfield
    Regular,
    NonRegular,
    SpawnFrame,
}

// impl Message for Command {}
// impl Message for CommandField {}
// impl Message for BrokerCommand {}
