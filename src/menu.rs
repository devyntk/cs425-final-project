use crate::{Message, User};
use postgres::Client;
use iced::{Column, Element, Text, Button};
use iced::button::State;

#[derive(Debug,Clone)]
pub enum MenuMessage {
    LogOut,
    EmployeeList
}

#[derive(Debug, Clone, Default)]
pub struct MenuState {
    log_out_state: State,
    list_state: State
}

impl MenuState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn update(&mut self, msg: MenuMessage, _client: &mut Client) -> Option<Message> {
        return match msg {
            MenuMessage::LogOut => {
                Some(Message::LogOut)
            }
            MenuMessage::EmployeeList => {
                Some(Message::EmployeeListMessage(crate::employee_list::EmployeeListMessage::Load))
            }
            _ => {None}
        }
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("Welcome, {}", user.username)))
            .push(Button::new(&mut self.log_out_state, Text::new("Log out"))
                .on_press(Message::MenuMessage(MenuMessage::LogOut)))
            .push(Button::new(&mut self.list_state, Text::new("View All Employees"))
                .on_press(Message::MenuMessage(MenuMessage::EmployeeList)))
            .into()
    }
}
