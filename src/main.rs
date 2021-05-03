use iced::{Sandbox, Element, Settings, Column, Text, Container, Button, button};
use postgres::{Client, NoTls};

fn main() -> iced::Result {
    env_logger::init();

    EmployeeDB::run(Settings::default())
}

struct EmployeeDB {
    user: UserType,
    page: Page,
    sql_client: Client,
    confirm_button: button::State
}

#[derive(Debug, Clone)]
enum Page {
    Main
}

#[derive(Debug, Clone)]
enum Message {
    SelectPage(Page)
}

#[derive(Debug, PartialEq)]
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
            confirm_button: button::State::default()
        }
    }

    fn title(&self) -> String {
        "CS425 Final Project".parse().unwrap()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        if self.user == UserType::None {
            let column = Column::new()
                .push(Text::new("Please select a user type from the following."))
                .push(
                    Button::new(&mut self.confirm_button, Text::new("Administrator"))
                );
            return column.into()
        }
        Column::new()
            .push(Text::new("Logged in."))
            .into()
    }
}
