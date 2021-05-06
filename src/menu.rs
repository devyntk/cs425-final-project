use crate::{Message, User};
use postgres::Client;
use iced::{Column, Element, Text, Button};
use iced::button::State;

#[derive(Debug,Clone)]
pub enum MenuMessage {
    LogOut
}

#[derive(Debug, Clone)]
pub struct MenuState {
    log_out_state: State
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            log_out_state: State::default()
        }
    }

    pub(crate) fn update(&mut self, msg: MenuMessage, _client: &mut Client) -> Option<Message> {
        return match msg {
            MenuMessage::LogOut => {
                Some(Message::LogOut)
            }
        }
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("Welcome, {}", user.username)))
            .push(Button::new(&mut self.log_out_state, Text::new("Log out"))
                .on_press(Message::MenuMessage(MenuMessage::LogOut)))
            .into()
    }
}
