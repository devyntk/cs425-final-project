mod employee;
mod employee_expense;
mod employee_list;
mod employee_year;
mod login;
mod menu;
mod paycheck;
mod w2;

use iced::{Element, Sandbox, Settings};
use log::info;
use postgres::{Client, NoTls};
use postgres_types::{FromSql, ToSql};

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
    employee_state: employee::EmployeeState,
    employee_year_state: employee_year::EmployeeYearState,
    employee_expense_state: employee_expense::EmployeeExpenseState,
    paycheck_state: paycheck::PaycheckState,
    w2_state: w2::W2State,
    employee_list_state: employee_list::EmployeeListState,
}

#[derive(Debug, Clone)]
enum Page {
    Main,
    Login,
    ViewEmployee,
    ViewEmployeeYear,
    EmployeeExpense,
    Paycheck,
    W2,
    EmployeeList,
}

#[derive(Debug, Clone)]
enum Message {
    SelectPage(Page),
    LoginMessage(login::LoginMessage),
    MenuMessage(menu::MenuMessage),
    EmployeeMessage(employee::EmployeeMessage),
    EmployeeYearMessage(employee_year::EmployeeYearMessage),
    EmployeeExpenseMessage(employee_expense::EmployeeExpenseMessage),
    PaycheckMessage(paycheck::PaycheckMessage),
    W2Message(w2::W2Message),
    EmployeeListMessage(employee_list::EmployeeListMessage),
    LogUser(User),
    LogOut,
}

#[derive(Debug, Clone)]
struct User {
    usertype: UserType,
    username: String,
    e_id: i32,
}
impl User {
    fn is_manager(&self) -> bool {
        match self.usertype {
            UserType::Manager | UserType::Administrator => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, ToSql, FromSql)]
#[postgres(name = "usertype")]
enum UserType {
    #[postgres(name = "employee")]
    Employee,
    #[postgres(name = "manager")]
    Manager,
    #[postgres(name = "admin")]
    Administrator,
}
impl Default for UserType {
    fn default() -> Self {
        Self::Employee
    }
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
            employee_state: employee::EmployeeState::new(),
            employee_year_state: employee_year::EmployeeYearState::new(),
            employee_expense_state: employee_expense::EmployeeExpenseState::new(),
            paycheck_state: paycheck::PaycheckState::new(),
            w2_state: w2::W2State::new(),
            employee_list_state: employee_list::EmployeeListState::new(),
        }
    }

    fn title(&self) -> String {
        "CS425 Final Project".parse().unwrap()
    }

    fn update(&mut self, message: Self::Message) {
        info!("{:?}", message);
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
                if let Some(msg) = self.employee_state.update(
                    msg,
                    &mut self.sql_client,
                    self.user.as_ref().unwrap(),
                ) {
                    self.update(msg)
                }
            }
            Message::EmployeeYearMessage(msg) => {
                if let Some(msg) = self.employee_year_state.update(
                    msg,
                    &mut self.sql_client,
                    self.user.as_ref().unwrap(),
                ) {
                    self.update(msg)
                }
            }
            Message::EmployeeExpenseMessage(msg) => {
                if let Some(msg) = self.employee_expense_state.update(
                    msg,
                    &mut self.sql_client,
                    self.user.as_ref().unwrap(),
                ) {
                    self.update(msg)
                }
            }
            Message::PaycheckMessage(msg) => {
                if let Some(msg) = self.paycheck_state.update(
                    msg,
                    &mut self.sql_client,
                    self.user.as_ref().unwrap(),
                ) {
                    self.update(msg)
                }
            }
            Message::W2Message(msg) => {
                if let Some(msg) =
                    self.w2_state
                        .update(msg, &mut self.sql_client, self.user.as_ref().unwrap())
                {
                    self.update(msg)
                }
            }
            Message::EmployeeListMessage(msg) => {
                if let Some(msg) = self.employee_list_state.update(
                    msg,
                    &mut self.sql_client,
                    self.user.as_ref().unwrap(),
                ) {
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
                                self.user.as_ref().expect("No ID To Load").e_id,
                            ),
                        ));
                        self.page = Page::ViewEmployee
                    }
                    _ => self.page = Page::Main,
                }
            }
            Message::LogOut => {
                self.user = None;
                self.page = Page::Login;
            }
            Message::SelectPage(page) => {
                self.page = page;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &self.page {
            Page::Main => self.menu_state.view(self.user.as_ref().unwrap()),
            Page::Login => self.login_state.view(),
            Page::ViewEmployee => self.employee_state.view(self.user.as_ref().unwrap()),
            Page::ViewEmployeeYear => self.employee_year_state.view(self.user.as_ref().unwrap()),
            Page::EmployeeList => self.employee_list_state.view(self.user.as_ref().unwrap()),
            Page::EmployeeExpense => self
                .employee_expense_state
                .view(self.user.as_ref().unwrap()),
            Page::W2 => self.w2_state.view(self.user.as_ref().unwrap()),
            Page::Paycheck => self.paycheck_state.view(self.user.as_ref().unwrap()),
        }
    }
}
