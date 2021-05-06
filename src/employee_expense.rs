use crate::{Message, Page, User};
use iced::button;
use iced::{Button, Column, Element, Text};

use log::info;
use postgres::Client;

#[derive(Debug, Clone)]
pub enum EmployeeExpenseMessage {
    Back,
    Load(i32),
}
#[derive(Debug, Clone, Default)]
pub struct EmployeeExpenseState {
    back_state: button::State,
    add_state: button::State,
    wages: f32,
    bonus: f32,
    retirement: f32,
    ssn_contribution: f32,
    insurance: f32,
}

impl EmployeeExpenseState {
    pub fn new() -> Self {
        EmployeeExpenseState::default()
    }

    pub(crate) fn update(
        &mut self,
        msg: EmployeeExpenseMessage,
        client: &mut Client,
        _user: &User,
    ) -> Option<Message> {
        match msg {
            EmployeeExpenseMessage::Load(year) => {
                self.wages = client
                    .query_one("SELECT find_wages($1)", &[&year])
                    .unwrap()
                    .get("find_wages");
                self.bonus = client
                    .query_one("SELECT bonus_paid($1)", &[&year])
                    .unwrap()
                    .get("bonus_paid");
                self.retirement = client
                    .query_one("SELECT retirement_employer($1)", &[&year])
                    .unwrap()
                    .get("retirement_employer");
                self.ssn_contribution = client
                    .query_one("SELECT ssn_employer($1)", &[&year])
                    .unwrap()
                    .get("ssn_employer");
                self.insurance = client
                    .query_one("SELECT insurance_employer($1)", &[&year])
                    .unwrap()
                    .get("insurance_employer");
                info!("{:?}", self);
                return Some(Message::SelectPage(Page::EmployeeExpense));
            }
            EmployeeExpenseMessage::Back => return Some(Message::SelectPage(Page::Main)),
        }
    }

    pub(crate) fn view(&mut self, _user: &User) -> Element<Message> {
        Column::new()
            .push(Text::new("Expense by year"))
            .push(
                Button::new(&mut self.back_state, Text::new("Back")).on_press(
                    Message::EmployeeExpenseMessage(EmployeeExpenseMessage::Back),
                ),
            )
            .push(Text::new(format!("Wages: {}", self.wages)))
            .push(Text::new(format!("Bonus: {}", self.bonus)))
            .push(Text::new(format!("Retirement: {}", self.retirement)))
            .push(Text::new(format!(
                "SSN Contribution: {}",
                self.ssn_contribution
            )))
            .push(Text::new(format!("Insurance: {}", self.insurance)))
            .push(Text::new(format!(
                "Total: {}",
                (self.insurance
                    + self.ssn_contribution
                    + self.retirement
                    + self.bonus
                    + self.wages)
            )))
            .into()
    }
}
