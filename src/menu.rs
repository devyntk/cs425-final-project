use crate::{Message, User};
use postgres::Client;
use iced::{Column, Element, Text, Button, text_input, Row};
use iced::button::State;
use crate::employee_expense::EmployeeExpenseMessage;

#[derive(Debug,Clone)]
pub enum MenuMessage {
    LogOut,
    EmployeeList,
    ChangeYear(String)
}
fn make_wrapper(variant: impl Fn(String) -> MenuMessage) -> impl Fn(String) -> Message{
    move |s| Message::MenuMessage(variant(s))
}

#[derive(Debug, Clone, Default)]
pub struct MenuState {
    log_out_state: State,
    list_state: State,
    add_employee_state: State,
    expense_state: State,
    expense_year: i32,
    expense_list_state: text_input::State
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
            MenuMessage::ChangeYear(str) => {
                self.expense_year = str.parse().unwrap_or(self.expense_year);
                None
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
            .push(Button::new(&mut self.add_employee_state, Text::new("Add New Employee"))
                .on_press(Message::EmployeeMessage(crate::employee::EmployeeMessage::CreateEmployee)))
            .push(
                Row::new()
                    .push(Text::new("Year for Expense Report:"))
                    .push(text_input::TextInput::new(
                        &mut self.expense_list_state,
                        "2021",
                        &*self.expense_year.to_string(),
                        make_wrapper(MenuMessage::ChangeYear)
                    ))
                    .push(Button::new(&mut self.expense_state,
                        Text::new("View Report"))
                        .on_press(Message::EmployeeExpenseMessage(EmployeeExpenseMessage::company_employee_expense(self.expense_year))))
            )
            .into()
    }
}
