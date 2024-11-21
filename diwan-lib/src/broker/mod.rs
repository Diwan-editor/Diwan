use std::collections::HashMap;
use std::sync::{Arc , Mutex};
use tokio::task::JoinHandle;

pub struct Broker {
    other_handlers: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
    widget_handlers: Arc<Mutex<HashMap<String, JoinHandle<()>>>>
}

impl Broker {
    // pub fn new() -> Self {
    //     Self {
    //         other_handlers: Arc::new(Mutex::new(

    //         )), widget_handlers:
    //     }
    // }
}
