use std::ops::{Deref, DerefMut};
use termwiz::widgets::Ui;

pub struct SendableUi<'a>(Ui<'a>);

impl<'a> SendableUi<'a> {
    pub fn new(ui: Ui<'a>) -> Self {
        SendableUi(ui)
    }
}

impl<'a> Deref for SendableUi<'a> {
    type Target = Ui<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for SendableUi<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<'a> Send for SendableUi<'a> {}
unsafe impl<'a> Sync for SendableUi<'a> {}
