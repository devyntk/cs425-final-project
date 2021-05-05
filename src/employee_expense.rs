use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum EmployeeExpenseMessage {
    company_employee_expense(i32)
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeExpenseMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeExpenseMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct EmployeeExpenseState {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    wages: i32,
    bonus: i32,
    retirement: i32,
    ssn_contribution: i32,
    insurance: i32,
}
impl EmployeeExpenseState {
    pub fn new() -> Self {
        EmployeeExpenseState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeExpenseMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            EmployeeExpenseMessage::company_employee_expense(year) => {
                let employee =client.query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id])
                    .expect("Can't find employee!");
                let wages = client.query("SELECT find_wages($1, $2)", &[&e_id, &report_year]);
                let bonus = client.query("SELECT bonus_paid($1, $2)", &[&e_id, &report_year]);
                let retirement = client.query("SELECT retirement_employer($1, $2)", &[&e_id, &report_year]);
                let ssn_contribution = client.query("SELECT ssn_employer($1, $2)", &[&e_id, &report_year]);
                let insurance = client.query("SELECT insurance_employer($1, $2)", &[&e_id, &report_year]);
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
