mod employee;
mod login;
mod menu;

use iced::{Column, Element, Sandbox, Settings};
use postgres::{Client, NoTls};
use crate::menu::MenuMessage;
use crate::Message::EmployeeMessage;

fn main() -> iced::Result {
    env_logger::init();

    EmployeeDB::run(Settings::default())
}

struct EmployeeDB {
    user: Option<User>,
    page: Page,
    sql_client: Client,
    login_state: login::LoginState,
    menu_state: menu::MenuState,
    employee_state: employee::EmployeeState
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
    LoginMessage(login::LoginMessage),
    MenuMessage(menu::MenuMessage),
    EmployeeMessage(employee::EmployeeMessage),
    LogUser(User),
    LogOut
}

#[derive(Debug, Clone)]
struct User {
    usertype: UserType,
    username: String,
    user_id: i32,
    has_dependent: bool,
    e_id: i32
}

#[derive(Debug, PartialEq, Clone)]
enum UserType {
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
            login_state: login::LoginState::new(),
            menu_state: menu::MenuState::new(),
            employee_state: employee::EmployeeState::new()
        }
    }

    fn title(&self) -> String {
        "CS425 Final Project".parse().unwrap()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            // module message handlers
            // allows the module update methods to return a Option<Message> that can also be
            // handled here too
            Message::LoginMessage(msg) => {
                 if let Some(msg) = self.login_state.update(msg, &mut self.sql_client) {
                     self.update(msg)
                 }
            }
            Message::MenuMessage(msg) => {
                if let Some(msg) = self.menu_state.update(msg, &mut self.sql_client) {
                    self.update(msg)
                }
            }
            Message::EmployeeMessage(msg) => {
                if let Some(msg) = self.employee_state.update(msg, &mut self.sql_client) {
                    self.update(msg)
                }
            }

            //global message handlers
            Message::LogUser(user) => {
                self.user = Some(user);
                match self.user.as_ref().unwrap().usertype {
                    UserType::Employee => {
                        self.update(Message::EmployeeMessage(
                            employee::EmployeeMessage::LoadEmployee(
                                self.user.as_ref().expect("No ID To Load").e_id)
                            )
                        );
                        self.page = Page::ViewEmployee
                    }
                    _ => {
                        self.page = Page::Main
                    }
                }
            }
            Message::LogOut => {
                self.user = None;
                self.page = Page::Login;
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &self.page {
            Page::Main => {self.menu_state.view(self.user.as_ref().unwrap())}
            Page::Login => {self.login_state.view()}
            Page::ViewEmployee => {self.employee_state.view(self.user.as_ref().unwrap())}
            _ => {Column::new().into()}
        }
    }
}
