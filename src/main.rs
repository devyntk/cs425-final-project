use iced::{Sandbox, Element, Settings, Column, Text, Button, button};
use postgres::{Client, NoTls};
use crate::Message::LogUser;

fn main() -> iced::Result {
    env_logger::init();

    EmployeeDB::run(Settings::default())
}

struct EmployeeDB {
    user: UserType,
    page: Page,
    sql_client: Client,
    confirm_button: button::State,
    back_button: button::State,
    forward_button: button::State
}

#[derive(Debug, Clone)]
enum Page {
    Main
}

#[derive(Debug, Clone)]
enum Message {
    SelectPage(Page),
    LogUser(UserType)
}

#[derive(Debug, PartialEq, Clone)]
enum UserType {
    None,
    Employee,
    Manager,
    Administrator
}

impl Sandbox for EmployeeDB {
    type Message = Message;

    fn new() -> Self {
        EmployeeDB {
            user: UserType::None,
            page: Page::Main,
            sql_client: Client::connect("host=localhost user=cs425", NoTls).unwrap(),
            confirm_button: button::State::default(),
            back_button: button::State::default(),
            forward_button: button::State::default()
        }
    }

    fn title(&self) -> String {
        "CS425 Final Project".parse().unwrap()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            LogUser(user) => {self.user = user}
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        if self.user == UserType::None {
            let column = Column::new()
                .push(Text::new("Please select a user type from the following:"))
                .push(
                    Button::new(&mut self.confirm_button, Text::new("Administrator"))
                        .on_press(Message::LogUser(UserType::Administrator))
                )
                .push(
                    Button::new(&mut self.forward_button, Text::new("Manager"))
                        .on_press(Message::LogUser(UserType::Manager))
                )
                .push(
                    Button::new(&mut self.back_button, Text::new("Employee"))
                        .on_press(Message::LogUser(UserType::Employee))
                );
            return column.into()
        }
        match self.page {
            Page::Main => {
                Column::new()
                    .push(Text::new(format!("Logged in as {:?}", self.user)))
                    .push(
                        Button::new(&mut self.back_button, Text::new("Log Out"))
                            .on_press(Message::LogUser(UserType::None)))
                    .into()
            }
        }
    }
}
