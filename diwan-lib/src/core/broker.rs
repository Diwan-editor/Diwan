use anyhow::Error;
use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};

/// this file will be used to define the broker that will manage the processes
///

pub struct Broker<'a >{
    // u8 here is basically an id
    buffers: Vec<(u8, &'a BufferedTerminal<UnixTerminal>)>,
    receivers: Vec<(u8, BufferedTerminal<UnixTerminal>)>,
    senders: Vec<(u8, BufferedTerminal<UnixTerminal>)>,
    active_buffer: u8,
}

impl<'a> Broker<'a> {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            receivers: Vec::new(),
            senders: Vec::new(),
            active_buffer: 1,
        }
    }

    pub fn add_buffer(&self, buf: &'a BufferedTerminal<UnixTerminal>) -> Result<u8, Error> {
        let max_id = self
            .buffers
            .iter()
            .reduce(|acc, x| if acc.0 <= x.0 { x.0 } else { acc.0 })
            .unwrap_or(self.buffers.last().unwrap().0);

        self.buffers.append((max_id.clone() + 1, buf));
        Ok(max_id)
    }
}
