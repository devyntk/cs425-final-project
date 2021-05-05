use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum W2Message {
    W2_report(i32, i32)
}
fn make_wrapper(variant: impl Fn(String) -> W2Message) -> impl Fn(String) -> Message{
    move |s| Message::W2Message(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct W2State {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    job_title: String,
    state_address: String,
    report_year: i32,
    yearly_income: i32,
    deductions: i32,
    bonus: i32
}
impl W2State {
    pub fn new() -> Self {
        W2State::default()
    }

    pub(crate) fn update(&mut self, msg: W2Message, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            W2Message::W2_report(e_id, report_year) => {
                let employee =client.query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id])
                    .expect("Can't find employee!");
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.report_year = report_year;
                let income = client.query("SELECT yearly_income($1, $2)", &[&e_id, &report_year]);
                let deductions = client.query("SELECT deductions($1, $2)", &[&e_id, &report_year]);
                let bonus = client.query("SELECT bonus_earned($1, $2)", &[&e_id, &report_year]);
                let report = client.query("SELECT w2_report($1, $2)", &[&e_id, &report_year]);
                println!("Employee Name: {} {}", self.first_name, self.last_name);
                println!("ssn: {} ", self.ssn);
                println!("yearly income: {:?}", income);
                println!("deductions: {:?}", deductions);
                println!("Bonus: {:?} ", bonus);
                println!("EMPLOYEE W2: {:?}", w2_report);
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
