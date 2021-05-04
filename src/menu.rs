use crate::Message;
use postgres::Client;
use iced::{Column, Element};

#[derive(Debug,Clone)]
pub enum MenuMessage {
}
fn make_wrapper(variant: impl Fn(String) -> MenuMessage) -> impl Fn(String) -> Message{
    move |s| Message::MenuMessage(variant(s))
}

#[derive(Debug, Clone)]
pub struct MenuState {
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
        }
    }

    pub(crate) fn update(&mut self, msg: MenuMessage, client: &mut Client) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        Column::new()
            .into()
    }
}
