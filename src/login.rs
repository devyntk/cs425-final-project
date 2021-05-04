use iced::{text_input, Column, Element};
use iced::button;
use iced::Text;
use crate::Message;

#[derive(Debug, Clone)]
pub struct LoginState {
    username_state: text_input::State,
    pub username: String,
    password_state: text_input::State,
    pub password: String,
    login_button: button::State,
    err_text: String
}

#[derive(Debug,Clone)]
pub enum LoginMessage {

}

impl LoginState {
    pub fn new() -> Self {
        LoginState {
            username: "".into(),
            password: "".into(),
            username_state: text_input::State::default(),
            password_state: text_input::State::default(),
            login_button: button::State::default(),
            err_text: "Please log in below.".into()
        }
    }

    pub(crate) fn update(&self, msg: LoginMessage){

    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new(self.err_text.as_str()))
            .push(text_input::TextInput::new(&mut self.username_state, "username", &*self.username, Message::UpdateUsername))
            .into()
    }
}
