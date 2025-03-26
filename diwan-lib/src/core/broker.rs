use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender};

use anyhow::Error;
use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};
use super::sync::command::BrokerCommand;

/// this file will be used to define the broker that will manage the processes
///

pub struct Broker<'a >{
    // u8 here is basically an id
    // does using the tuple here wont trigger an error ? is it Sized ?
    buffers: Vec<(u8, &'a BufferedTerminal<UnixTerminal>)>,
    pub receiver: Option<Receiver<BrokerCommand>>,
    pub senders: Vec<(u8, Sender<BrokerCommand>)>,
    /// id of the active buffer
    active_buffer: u8,
}

impl<'a> Broker<'a> {
    pub async fn new(mut broker_receiver: Receiver<BrokerCommand>) -> Self {

        // tokio::spawn(async {
        //     let mut broker = Self {
        //         buffers: Vec::new(),
        //         senders: Vec::new(),
        //         receiver: broker_receiver.into(),
        //         active_buffer: 1,
        //     };

            loop {
                //broker.handle_command().await;
                if let Some(command) = broker_receiver.recv().await {
                    match command {
                        BrokerCommand::GetBuffers =>
                            println!("matched"),
                        _ =>
                            println!("something else "),
                    }
                }
            }

//}).await.unwrap()
    }

    async fn handle_command(&mut self) {
        let receiver = self.receiver.as_mut().unwrap();

        println!("Inside handle_command");
        if let Some(command) = receiver.recv().await {
            match command {
                BrokerCommand::GetBuffers =>
                    println!("matched"),
                _ =>
            println!("something else "),
            }
        }
    }

    pub fn add_buffer(&mut self, buf: &'a BufferedTerminal<UnixTerminal>) -> Result<u8, Error> {
        let max_id: u8 = self
            .buffers
            .iter()
            .map(|x| x.0)
            .reduce(|acc, x| if acc <= x { x } else { acc })
            .unwrap_or(self.buffers.last().unwrap().0);

        self.buffers.push((max_id.clone() + 1, buf));
        Ok(max_id)
    }

    pub fn kill_buffer(&mut self, buffer_id: u8) {
        // implement the removal of the frame from the rendering pane
        todo!();
    }

    pub fn get_buffers(&self) -> Result<&'a Vec<(u8, &BufferedTerminal<UnixTerminal>)>, Error> {

        Ok(&self.buffers)
    }

    pub fn spawn_frame(&self, buffer_id: u8) -> Result<u8, Error> {
        self
            .buffers
            .iter()
            .any(|buf| buf.0 == buffer_id)
            .then(|| todo!() ).or(None).expect("TODO")
    }
}
