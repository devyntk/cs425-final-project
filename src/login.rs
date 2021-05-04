use iced::{text_input, Column, Element};
use iced::button;
use iced::Text;
use crate::Message;
use postgres::{Client, Row};

#[derive(Debug,Clone)]
pub enum LoginMessage {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit
}
impl LoginMessage {
    // This is done individually for each message type because enum variants aren't their own
    // type, so we can't make this function generic. Hopefully a future rust allows this to clean
    // up but I don't plan on altering code past like. two days from now.
    fn wrap_username_message(str: String) -> Message {
        return Message::LoginMessage(LoginMessage::UpdateUsername(str))
    }
    fn wrap_password_message(str: String) -> Message {
        return Message::LoginMessage(LoginMessage::UpdatePassword(str))
    }

}

#[derive(Debug, Clone)]
pub struct LoginState {
    username_state: text_input::State,
    pub username: String,
    password_state: text_input::State,
    pub password: String,
    login_button: button::State,
    err_text: String,
    disabled: bool
}

impl LoginState {
    pub fn new() -> Self {
        LoginState {
            username: "".into(),
            password: "".into(),
            username_state: text_input::State::default(),
            password_state: text_input::State::default(),
            login_button: button::State::default(),
            err_text: "Please log in below.".into(),
            disabled: false
        }
    }

    pub(crate) fn update(&mut self, msg: LoginMessage, client: &mut Client){
        match msg {
            LoginMessage::UpdateUsername(username) => {
                self.username = username
            },
            LoginMessage::UpdatePassword(password) => {
                self.password = password
            },
            LoginMessage::Submit => {
                if self.disabled {
                    return
                }
                self.disabled = true;
                // do stuff
                let user = client.query_opt("SELECT * FROM user_tbl WHERE username = $1 AND psswrd = $2", &[&self.username, &self.password]);
                match user {
                    Ok(result) => {
                        match result {
                            None => {
                                self.err_text = "No Users Found".parse().unwrap()
                            }
                            Some(_) => {
                                self.err_text = "Logged in!".parse().unwrap();
                            }
                        }
                    },
                    Err(err) => {
                        self.err_text = err.to_string()
                    }
                }
                self.disabled = false;
            }
        }
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new(self.err_text.as_str()))
            .push(text_input::TextInput::new(&mut self.username_state, "username", &*self.username, LoginMessage::wrap_username_message))
            .push(text_input::TextInput::new(&mut self.password_state, "password", &*self.password, LoginMessage::wrap_password_message))
            .push(button::Button::new(&mut self.login_button, Text::new(
                match self.disabled{
                    true => {"Logging in..."}
                    false => {"Log In"}
                } ))
                .on_press(Message::LoginMessage(LoginMessage::Submit)))
            .into()
    }
}
