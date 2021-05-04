use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum PaycheckMessage {
}
fn make_wrapper(variant: impl Fn(String) -> PaycheckMessage) -> impl Fn(String) -> Message{
    move |s| Message::PaycheckMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct PaycheckState {

}
impl PaycheckState {
    pub fn new() -> Self {
        PaycheckState::default()
    }

    pub(crate) fn update(&mut self, msg: PaycheckMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
