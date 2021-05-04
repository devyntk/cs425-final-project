use iced::{text_input, Column, Element};
use iced::button;
use iced::Text;
use crate::{Message, User, UserType};
use postgres::Client;

#[derive(Debug,Clone)]
pub enum LoginMessage {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit
}
fn make_wrapper(variant: impl Fn(String) -> LoginMessage) -> impl Fn(String) -> Message{
    move |s| Message::LoginMessage(variant(s))
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

    pub(crate) fn update(&mut self, msg: LoginMessage, client: &mut Client) -> Option<Message> {
        match msg {
            LoginMessage::UpdateUsername(username) => {
                self.username = username;
            },
            LoginMessage::UpdatePassword(password) => {
                self.password = password;
            },
            LoginMessage::Submit => {
                if self.disabled {
                    return None;
                }
                self.disabled = true;
                let user = client.query_opt("SELECT * FROM user_tbl WHERE username = $1 AND psswrd = $2", &[&self.username, &self.password]);
                match user {
                    Ok(result) => {
                        match result {
                            None => {
                                self.err_text = "No Users Found".parse().unwrap()
                            }
                            Some(row) => {
                                self.disabled = false;
                                return Some(Message::LogUser(User{
                                    usertype: if row.get("isAdmin") {
                                        UserType::Administrator
                                    } else if row.get("IsEmployer"){
                                        UserType::Manager
                                    } else {
                                        UserType::Employee
                                    },
                                    username: row.get("username"),
                                    user_id: row.get("user_ID"),
                                    has_dependent: row.get("HasDependent")
                                }));

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
        None
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new(self.err_text.as_str()))
            .push(text_input::TextInput::new(&mut self.username_state, "username", &*self.username, make_wrapper(LoginMessage::UpdateUsername)))
            .push(text_input::TextInput::new(&mut self.password_state, "password", &*self.password, make_wrapper(LoginMessage::UpdatePassword)))
            .push(button::Button::new(&mut self.login_button, Text::new(
                match self.disabled{
                    true => {"Logging in..."}
                    false => {"Log In"}
                } ))
                .on_press(Message::LoginMessage(LoginMessage::Submit)))
            .into()
    }
}
