use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum EmployeeYearMessage {
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeYearMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeYearMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct EmployeeYearState {

}
impl EmployeeYearState {
    pub fn new() -> Self {
        EmployeeYearState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeYearMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
