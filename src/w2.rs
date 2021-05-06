use crate::{Message, Page, User};
use iced::button;
use iced::{text_input, Button, Column, Element, Row, Text, TextInput};

use postgres::Client;

#[derive(Debug, Clone)]
pub enum W2Message {
    W2Report { year: i32, e_id: i32 },
    Back,
}
fn make_wrapper(variant: impl Fn(String) -> W2Message) -> impl Fn(String) -> Message {
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
    report: f32,
    yearly_income: f32,
    deductions: f32,
    bonus: f32,
    logout_button: button::State,
}
impl W2State {
    pub fn new() -> Self {
        W2State::default()
    }

    pub(crate) fn update(
        &mut self,
        msg: W2Message,
        client: &mut Client,
        _user: &User,
    ) -> Option<Message> {
        match msg {
            W2Message::W2Report { e_id, year } => {
                let employee = client
                    .query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id])
                    .expect("Can't find employee!");
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.report_year = year;
                let income = client.query_one("SELECT yearly_income($1, $2)", &[&e_id, &year]);
                let deductions = client.query_one("SELECT deductions($1, $2)", &[&e_id, &year]);
                let bonus = client.query_one("SELECT bonus_earned($1, $2)", &[&e_id, &year]);
                let report = client.query_one("SELECT w2_report($1, $2)", &[&e_id, &year]);
                self.yearly_income = income.unwrap().get("yearly_income");
                self.deductions = deductions.unwrap().get("deductions");
                self.bonus = bonus.unwrap().get("bonus_earned");
                self.report = report.unwrap().get("w2_report");

                return Some(Message::SelectPage(Page::W2));
            }
            W2Message::Back => return Some(Message::SelectPage(Page::ViewEmployeeYear)),
        }
        None
    }

    pub(crate) fn view(&mut self, _user: &User) -> Element<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(Text::new("Employee Name:"))
                    .push(Text::new(&*self.first_name)),
            )
            .push(
                Row::new()
                    .push(Text::new("SSN: "))
                    .push(Text::new(&*self.ssn)),
            )
            .push(
                Row::new()
                    .push(Text::new("Yearly income: "))
                    .push(Text::new(format!("{:?}", self.yearly_income))),
            )
            .push(
                Row::new()
                    .push(Text::new("Deductions: "))
                    .push(Text::new(format!("{:?}", self.deductions))),
            )
            .push(
                Row::new()
                    .push(Text::new("Bonus: "))
                    .push(Text::new(format!("{:?}", self.bonus))),
            )
            .push(
                Row::new()
                    .push(Text::new("EMPLOYEE W2: "))
                    .push(Text::new(format!("{:?}", self.report))),
            )
            .push(
                Button::new(&mut self.logout_button, Text::new("Back To Employee Year"))
                    .on_press(Message::W2Message(W2Message::Back)),
            )
            .into()
    }
}
