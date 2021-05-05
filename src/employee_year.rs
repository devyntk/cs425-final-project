use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::{warn, info};
use std::error::Error;
use postgres_types::{ToSql, FromSql};

#[derive(Debug,Clone)]
pub enum EmployeeYearMessage {
    Load {
        year: i32,
        e_id: i32
    }
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeYearMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeYearMessage(variant(s))
}

#[derive(Debug, Clone, ToSql, FromSql)]
#[postgres(name="perfomance")]
enum Performance {
    #[postgres(name="well")]
    Well,
    #[postgres(name="ok")]
    Ok,
    #[postgres(name="not_well")]
    NotWell,
    #[postgres(name="super_performer")]
    SuperPerformer,
    #[postgres(name="manager")]
    Manager
}
impl Default for Performance {
    fn default() -> Self { Performance::Ok }
}

#[derive(Debug, Clone, ToSql, FromSql)]
#[postgres(name="salary")]
enum SalaryType {
    #[postgres(name="w2")]
    W2,
    #[postgres(name="hourly")]
    Hourly
}
impl Default for SalaryType {
    fn default() -> Self { SalaryType::W2 }
}

#[derive(Debug, Clone, Default)]
pub struct EmployeeYearState {
    editable: bool,
    e_id: i32,
    e_year: i32,
    salary: f32,
    salary_type: SalaryType,
    performance: Performance,
    social_security: Option<SocialSecurity>,
    benefits: Option<Benefits>,
    bonus: Option<Bonus>,
    insurance: Option<InsurancePlan>
}

#[derive(Debug, Clone, Default)]
struct SocialSecurity {
    amount: f32,
    employee_pays: f32,
    employer_pays: f32,
}

#[derive(Debug, Clone, Default)]
struct Benefits {
    benefit_type: String,
    employee_contribution: f32,
    employer_contribution: f32
}

#[derive(Debug, Clone, Default)]
struct Bonus {
    percentage: f32,
    company_sale: f32
}

#[derive(Debug, Clone, Default)]
struct InsurancePlan {
    insurance_type: String,
    premium: f32,
    employer_contribution: f32
}

impl EmployeeYearState {
    pub fn new() -> Self {
        EmployeeYearState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeYearMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            EmployeeYearMessage::Load {year, e_id} => {
                let employeeyr = client.query_one("SELECT * FROM employeeYear WHERE E_ID = $1 AND e_year = $2;", &[&e_id, &year])
                    .expect("Can't find employee!");
                self.e_id = employeeyr.get("E_ID");
                self.e_year = employeeyr.get("e_year");
                self.salary = employeeyr.get("salary");
                self.salary_type = employeeyr.get("salaryType");
                self.performance = employeeyr.get("performance");
                info!("{:?}", &self);
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .into()
    }
}
