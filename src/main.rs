mod employee;
mod login;

use crate::Message::LogUser;
use iced::{button, Button, Column, Element, Sandbox, Settings, Text};
use postgres::{Client, NoTls};
use crate::Page::Login;

fn main() -> iced::Result {
    env_logger::init();

    EmployeeDB::run(Settings::default())
}

struct EmployeeDB {
    user: Option<User>,
    page: Page,
    sql_client: Client,
    login_state: login::LoginState
}

#[derive(Debug, Clone)]
enum Page {
    Main,
    Login,
    ViewEmployee,
}

#[derive(Debug, Clone)]
enum Message {
    SelectPage(Page),
    LogUser(User),
    LoginMessage(login::LoginMessage),
    TestMessage(String)
}

#[derive(Debug, Clone)]
struct User {
    usertype: UserType,
    username: String,
    user_id: u32,
    has_dependent: bool
}

#[derive(Debug, PartialEq, Clone)]
enum UserType {
    None,
    Employee,
    Manager,
    Administrator,
}

impl Sandbox for EmployeeDB {
    type Message = Message;

    fn new() -> Self {
        EmployeeDB {
            user: None,
            page: Page::Login,
            sql_client: Client::connect("host=localhost user=cs425", NoTls).unwrap(),
            login_state: login::LoginState::new()
        }
    }

    fn title(&self) -> String {
        "CS425 Final Project".parse().unwrap()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LogUser(user) => self.user = Some(user),
            Message::LoginMessage(msg) => {
                self.login_state.update(msg, &mut self.sql_client)
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        if self.user.is_none() {
        }
        match &self.page {
            Page::Main => {Column::new().into()}
            Page::Login => {self.login_state.view()}
            _ => {Column::new().into()}
        }
    }
}
