use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::warn;

#[derive(Debug,Clone)]
pub enum PaycheckMessage {
    paycheck_report(i32, i32)
}
fn make_wrapper(variant: impl Fn(String) -> PaycheckMessage) -> impl Fn(String) -> Message{
    move |s| Message::PaycheckMessage(variant(s))
}


#[derive(Debug, Clone, Default)]
pub struct PaycheckState {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    state_address: String,
    medicare: i32,
    state_tax: i32,
    four_one_k: i32,
    insurance_premium: i32,
    report_year: i32,
    logout_button: button::State
}
impl PaycheckState {
    pub fn new() -> Self {
        PaycheckState::default()
    }

    pub(crate) fn update(&mut self, msg: PaycheckMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            PaycheckMessage::paycheck_report(e_id, report_year) => {
                let employee =client.query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id])
                    .expect("Can't find employee!");
                let statetax = client.query("SELECT stateTax($1, $2)", &[&e_id, &report_year]);
                let brackettax = client.query("SELECT bracket($1, $2)", &[&e_id, &report_year]);
                let four_one_k = client.query("SELECT Val401k($1)", &[&e_id]);
                let social_sec = client.query("SELECT socialSec($1, $2)", &[&e_id, &report_year]);
                let insurance = client.query("SELECT insurance_premium($1, $2)", &[&e_id, &report_year]);
                let medicare = client.query("SELECT medicare($1, $2)", &[&e_id, &report_year]);
                let paycheck = client.query("SELECT paycheck($1, $2)", &[&e_id, &report_year]);
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.state_address = employee.get( "stateAddress");
                self.report_year = report_year;
                /*println!("Employee Name: {} {}", self.first_name, self.last_name);
                println!("ssn: {} ", self.ssn);
                println!("Tax Deductions: {:?} [state tax], {:?} [federal tax], {:?} [social security] {:?} [medicare]", statetax, brackettax, social_sec, medicare);
                println!("401k contribution: {:?}", four_one_k);
                println!("insurance premium: {:?}", insurance);
                println!("EMPLOYEE PAYCHECK: {:?}", paycheck);*/
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(Row::new()
                .push(Text::new("Employee Name:"))
                .push(Text::new(&*self.first_name)))
            .push(Row::new()
                .push(Text::new("SSN: "))
                .push(Text::new(&*self.ssn)))
            // .push(Row::new()
            //     .push(Text::new("Tax Deductions: "))
            //     .push(Text::new(&self.statetax))
            //     .push(Text::new(&self.brackettax)))
            // .push(Row::new()
            //     .push(Text::new("401k contribution: "))
            //     .push(Text::new(&self.fouronek)))
            // .push(Row::new()
            //     .push(Text::new("Insurance Premium contribution: "))
            //     .push(Text::new(&self.insurance)))
            // .push(Row::new()
            //     .push(Text::new("EMPLOYEE PAYCHECK: "))
            //     .push(Text::new(&self.paycheck)))
            .push(match user.usertype {
                UserType::Manager => {
                    Button::new(&mut self.logout_button, Text::new("Log Out"))
                        .on_press(Message::LogOut)
                }
                _ => {
                    Button::new(&mut self.logout_button, Text::new("Back to Menu"))
                        .on_press(Message::SelectPage(crate::Page::Main))

                }
            })
            .into()
    }
}
