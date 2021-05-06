use crate::{Message, Page, User, UserType};
use iced::button;
use iced::{text_input, Button, Column, Element, Row, Text, TextInput};
use log::warn;
use postgres::Client;

#[derive(Debug, Clone)]
pub enum PaycheckMessage {
    Load { year: i32, e_id: i32 },
    Back,
}
fn make_wrapper(variant: impl Fn(String) -> PaycheckMessage) -> impl Fn(String) -> Message {
    move |s| Message::PaycheckMessage(variant(s))
}

#[derive(Debug, Clone, Default)]
pub struct PaycheckState {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    state_address: String,
    medicare: f32,
    state_tax: f32,
    social_sec: f32,
    bracket_tax: f32,
    four_one_k: f32,
    insurance_premium: f32,
    report: f32,
    report_year: i32,
    logout_button: button::State,
}
impl PaycheckState {
    pub fn new() -> Self {
        PaycheckState::default()
    }

    pub(crate) fn update(
        &mut self,
        msg: PaycheckMessage,
        client: &mut Client,
        user: &User,
    ) -> Option<Message> {
        match msg {
            PaycheckMessage::Load { e_id, year } => {
                let employee = client
                    .query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id])
                    .expect("Can't find employee!");
                let statetax = client.query_one("SELECT stateTax($1, $2)", &[&e_id, &year]);
                let brackettax = client.query_one("SELECT bracket($1, $2)", &[&e_id, &year]);
                let four_one_k = client.query_one("SELECT Val401k($1)", &[&e_id]);
                let social_sec = client.query_one("SELECT socialSec($1, $2)", &[&e_id, &year]);
                let insurance =
                    client.query_one("SELECT insurance_premium($1, $2)", &[&e_id, &year]);
                let medicare = client.query_one("SELECT medicare($1, $2)", &[&e_id, &year]);
                let paycheck = client.query_one("SELECT paycheck($1, $2)", &[&e_id, &year]);
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.state_address = employee.get("stateAddress");
                self.report_year = year;
                self.state_tax = statetax.unwrap().get("stateTax");
                self.four_one_k = four_one_k.unwrap().get("Val401k");
                self.bracket_tax = brackettax.unwrap().get("bracket");
                self.social_sec = social_sec.unwrap().get("socialSec");
                self.insurance_premium = insurance.unwrap().get("insurance_premium");
                self.medicare = medicare.unwrap().get("medicare");
                self.report = paycheck.unwrap().get("paycheck");

                return Some(Message::SelectPage(Page::Paycheck));
            }
            PaycheckMessage::Back => return Some(Message::SelectPage(Page::Main)),
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(Text::new("Employee Name:"))
                    .push(Text::new(&self.first_name)),
            )
            .push(
                Row::new()
                    .push(Text::new("SSN: "))
                    .push(Text::new(&self.ssn)),
            )
            .push(
                Row::new()
                    .push(Text::new("Tax Deductions: "))
                    .push(Text::new(format!("{:?}", self.state_tax)))
                    .push(Text::new(format!("{:?}", self.bracket_tax))),
            )
            .push(
                Row::new()
                    .push(Text::new("401k contribution: "))
                    .push(Text::new(format!("{:?}", self.four_one_k))),
            )
            .push(
                Row::new()
                    .push(Text::new("Social Security contribution: "))
                    .push(Text::new(format!("{:?}", self.social_sec))),
            )
            .push(
                Row::new()
                    .push(Text::new("Insurance Premium contribution: "))
                    .push(Text::new(format!("{:?}", self.insurance_premium))),
            )
            .push(
                Row::new()
                    .push(Text::new("EMPLOYEE PAYCHECK: "))
                    .push(Text::new(format!("{:?}", self.report))),
            )
            .push(match user.usertype {
                UserType::Manager => Button::new(&mut self.logout_button, Text::new("Log Out"))
                    .on_press(Message::LogOut),
                _ => Button::new(&mut self.logout_button, Text::new("Back to Menu"))
                    .on_press(Message::SelectPage(crate::Page::Main)),
            })
            .into()
    }
}
