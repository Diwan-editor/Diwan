use std::{future::IntoFuture, sync::Arc};

use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::screen::DWidget;

use super::sync::command::BrokerCommand;
use anyhow::{anyhow, Error};
use termwiz::{
    terminal::{buffered::BufferedTerminal, UnixTerminal},
    widgets,
};

/// this file will be used to define the broker that will manage the processes
pub struct Broker {
    // u8 here is basically an id
    // does using the tuple here wont trigger an error ? is it Sized ?
    buffers: Vec<(u8, Arc<BufferedTerminal<UnixTerminal>>, DWidget)>,
    pub receiver: Option<Receiver<BrokerCommand>>,
    pub senders: Vec<(u8, Sender<BrokerCommand>)>,
    /// id of the active buffer
    active_buffer: u8,
}

impl Broker {
    // unused mut
    // NOTE(rename): in case the new function doesn't return a value or Self better to name it run then!
    pub async fn new(broker_receiver: Receiver<BrokerCommand>) {
        let mut broker = Self {
            buffers: Vec::new(),
            senders: Vec::new(),
            receiver: broker_receiver.into(),
            active_buffer: 1,
        };

        loop {
            broker.handle_command().await;
        }
    }

    // TODO: i should define later a union of Command types
    async fn handle_command(&mut self) {
        let receiver = self.receiver.as_mut().unwrap();

        if let Some(command) = receiver.recv().await {
            match command {
                BrokerCommand::GetBuffers(sender_id) => {
                    let buffers = self.get_buffers().unwrap();
                    let sender_tx = self.get_sender(sender_id).unwrap();
                    sender_tx
                        .send(BrokerCommand::OkBuffers(buffers))
                        .await
                        .unwrap();
                }
                BrokerCommand::Mes(_id, _, _text) => {
                    println!("matched, MES");
                }
                //BrokerCommand::SpawnBuffer(id) => todo!(),
                BrokerCommand::AddBuffer(sender_id, (buffered_terminal, widget)) => {
                    let new_buffer_id: u8 = self.add_buffer(buffered_terminal, widget).unwrap();
                    let sender_tx = self.get_sender(sender_id).unwrap();
                    sender_tx
                        .send(BrokerCommand::Ok(new_buffer_id))
                        .await
                        .unwrap();
                }
                BrokerCommand::SignalExit(id) => todo!(),
                other => {
                    println!("other non implemented stuffs ");
                }
            }
        }
    }

    pub fn add_buffer(
        &mut self,
        buf: BufferedTerminal<UnixTerminal>,
        widget: DWidget,
    ) -> Result<u8, Error> {
        let max_id: u8 = self
            .buffers
            .iter()
            .map(|x| x.0)
            .reduce(|acc, x| if acc <= x { x } else { acc })
            .unwrap_or(self.buffers.last().unwrap().0);

        self.buffers.push((max_id + 1, buf.into(), widget));
        Ok(max_id)
    }

    pub fn kill_buffer(&mut self, buffer_id: u8) {
        // implement the removal of the frame from the rendering pane
        todo!();
    }

    pub fn get_buffers(
        &self,
    ) -> Result<Vec<(u8, Arc<BufferedTerminal<UnixTerminal>>, DWidget)>, Error> {
        Ok(self.buffers.clone()) // ✅ clones the Vec and its contents (Arc makes it cheap)
    }
    pub fn spawn_frame(&self, buffer_id: u8) -> Result<u8, Error> {
        self.buffers
            .iter()
            .any(|buf| buf.0 == buffer_id)
            .then(|| todo!())
            .or(None)
            .expect("TODO")
    }

    fn get_sender(&self, sender_id: u8) -> Result<Sender<BrokerCommand>, Error> {
        self.senders
            .iter()
            .find(|item| item.0 == sender_id)
            .map(|item| item.1.clone())
            .ok_or_else(|| anyhow!("Sender not found"))
    }
}
