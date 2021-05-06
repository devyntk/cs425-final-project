use crate::{Message, User, UserType, Page};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum EmployeeListMessage {
    Back,
    Edit(i32),
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
    edit_state: button::State
}
impl EmployeeListEntry {
    fn view(&mut self) -> Row<Message> {
        Row::new().push(Text::new(format!("{} {} ({})", self.first_name, self.last_name, self.e_id)))
            .push(Button::new(&mut self.edit_state, Text::new("Edit"))
                .on_press(Message::EmployeeListMessage(EmployeeListMessage::Edit(self.e_id))))
    }
}

impl EmployeeListState {
    pub fn new() -> Self {
        EmployeeListState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeListMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            EmployeeListMessage::Load => {
                let emp_list = client.query("SELECT * FROM employee", &[]);
                for emp in emp_list.expect("Cannot Get employees!") {
                    self.entries.insert(emp.get("e_id"), EmployeeListEntry {
                        first_name: emp.get("firstName"),
                        last_name: emp.get("lastName"),
                        e_id: emp.get("E_ID"),
                        edit_state: Default::default()
                    });
                }
                return Some(Message::SelectPage(Page::EmployeeList))
            }
            EmployeeListMessage::Back => {
                return Some(Message::SelectPage(Page::Main))
            }
            EmployeeListMessage::Edit(idx) => {
                return Some(Message::EmployeeMessage(crate::employee::EmployeeMessage::LoadEmployee(idx)))
            }
            EmployeeListMessage::AddEmployee => {
                return Some(Message::EmployeeMessage(crate::employee::EmployeeMessage::CreateEmployee))
            }
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
            .push(Button::new(&mut self.back_state, Text::new("Back"))
                .on_press(Message::EmployeeListMessage(EmployeeListMessage::Back)))
            .push(Button::new(&mut self.add_state, Text::new("Add Employee"))
                .on_press(Message::EmployeeListMessage(EmployeeListMessage::AddEmployee)))
            .into()
    }
}
