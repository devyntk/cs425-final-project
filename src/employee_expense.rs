use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum EmployeeExpenseMessage {
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeExpenseMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeExpenseMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct EmployeeExpenseState {

}
impl EmployeeExpenseState {
    pub fn new() -> Self {
        EmployeeExpenseState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeExpenseMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
