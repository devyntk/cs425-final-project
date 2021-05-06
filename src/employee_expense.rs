use crate::{Message, Page, User, UserType};
use iced::button;
use iced::{text_input, Button, Column, Element, Row, Text, TextInput};
use log::warn;
use postgres::Client;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EmployeeExpenseMessage {
    Back,
    Load(i32),
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeExpenseMessage) -> impl Fn(String) -> Message {
    move |s| Message::EmployeeExpenseMessage(variant(s))
}

#[derive(Debug, Clone, Default)]
pub struct EmployeeExpenseState {
    entries: HashMap<i32, EmployeeExpenseEntry>,
    back_state: button::State,
    add_state: button::State,
}

#[derive(Debug, Clone, Default)]
struct EmployeeExpenseEntry {
    e_id: i32,
    //ssn: String,
    // first_name: String,
    // last_name: String,
    wages: i32,
    bonus: i32,
    retirement: i32,
    ssn_contribution: i32,
    insurance: i32,
}

impl EmployeeExpenseEntry {
    fn view(&mut self) -> Row<Message> {
        Row::new().push(Text::new(format!(
            "{}, {:?}, {:?}, {:?}, {:?}, {:?}",
            self.e_id,
            self.wages,
            self.bonus,
            self.retirement,
            self.ssn_contribution,
            self.insurance
        )))
    }
}
impl EmployeeExpenseState {
    pub fn new() -> Self {
        EmployeeExpenseState::default()
    }

    pub(crate) fn update(
        &mut self,
        msg: EmployeeExpenseMessage,
        client: &mut Client,
        user: &User,
    ) -> Option<Message> {
        match msg {
            EmployeeExpenseMessage::Load(year) => {
                let emps = client.query("SELECT * FROM employeeYear WHERE e_year = $1;", &[&year]);
                for emp in emps.expect("Cannot Find Years") {
                    let e_id: i32 = emp.get("e_id");
                    let wages = client.query_one("SELECT find_wages($1)", &[&e_id]);
                    let bonus = client.query_one("SELECT bonus_paid($1)", &[&e_id]);
                    let retirement = client.query_one("SELECT retirement_employer($1)", &[&e_id]);
                    let ssn_contribution = client.query_one("SELECT ssn_employer($1)", &[&e_id]);
                    let insurance = client.query_one("SELECT insurance_employer($1, $2)", &[&e_id]);
                    let expenses = client.query_one("SELECT find_wages($1) inner join", &[&e_id]);
                    self.entries.insert(
                        e_id,
                        EmployeeExpenseEntry {
                            e_id,
                            wages: wages.unwrap().get("salary_val"),
                            bonus: bonus.unwrap().get("bonus"),
                            retirement: retirement.unwrap().get("amount"),
                            ssn_contribution: ssn_contribution.unwrap().get("amount"),
                            insurance: insurance.unwrap().get("amount"),
                        },
                    );
                }
                return Some(Message::SelectPage(Page::EmployeeExpense));
            }
            EmployeeExpenseMessage::Back => return Some(Message::SelectPage(Page::Main)),
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(Text::new("Expense by year"))
            .push(
                self.entries
                    .iter_mut()
                    .fold(Column::new(), |parent: Column<Message>, (year, entry)| {
                        parent.push(entry.view())
                    }),
            )
            .push(
                Button::new(&mut self.back_state, Text::new("Back")).on_press(
                    Message::EmployeeExpenseMessage(EmployeeExpenseMessage::Back),
                ),
            )
            .into()
    }
}
