use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum EmployeeListMessage {
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeListMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeListMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct EmployeeListState {

}
impl EmployeeListState {
    pub fn new() -> Self {
        EmployeeListState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeListMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
