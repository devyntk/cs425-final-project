use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum EmployeeListMessage {
    Back,
    Edit(i32),
    W2(i32),
    Paycheck(i32),
    Load,
    AddEmployee
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeListMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeListMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct EmployeeListState {
    entries: HashMap<i32, EmployeeListEntry>,
    back_state: button::State,
    add_state: button::State,
}

#[derive(Debug, Clone, Default)]
struct EmployeeListEntry{
    first_name: String,
    last_name: String,
    e_id: i32,
    edit_state: button::State,
    w2_state: button::State,
    paycheck_state: button::State
}
impl EmployeeListEntry {
    fn view(&mut self) -> Row<Message> {
        Row::new()
    }
}

impl EmployeeListState {
    pub fn new() -> Self {
        EmployeeListState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeListMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            EmployeeListMessage::Back => {}
            EmployeeListMessage::Edit(_) => {}
            EmployeeListMessage::W2(_) => {}
            EmployeeListMessage::Paycheck(_) => {}
            EmployeeListMessage::Load => {}
            EmployeeListMessage::AddEmployee => {}
        }
        None
    }

    pub(crate) fn view(&mut self, _user: &User) -> Element<Message> {
        Column::new()
            .push(Text::new("All Employees"))
            .push(self.entries.iter_mut().fold(
                Column::new(),
                |parent: Column<Message>, (d_id, entry)| {
                    parent.push(entry.view())}))
            .into()
    }
}
