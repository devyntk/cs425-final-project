use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum W2Message {
}
fn make_wrapper(variant: impl Fn(String) -> W2Message) -> impl Fn(String) -> Message{
    move |s| Message::W2Message(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct W2State {

}
impl W2State {
    pub fn new() -> Self {
        W2State::default()
    }

    pub(crate) fn update(&mut self, msg: W2Message, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
