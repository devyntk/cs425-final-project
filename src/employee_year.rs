use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input, pick_list};
use iced::button;
use log::{warn, info};
use std::error::Error;
use postgres_types::{ToSql, FromSql};

#[derive(Debug,Clone)]
pub enum EmployeeYearMessage {
    Load {
        year: i32,
        e_id: i32
    },
    DummyUpdate(String),
    UpdateSalaryType(SalaryType),
    UpdatePerformance(Performance),
    UpdateSalary(String),
    UpdateSSAmount(String),
    UpdateSSEmployeePays(String),
    UpdateSSEmployerPays(String),
    CreateSS
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeYearMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeYearMessage(variant(s))
}
fn make_wrapper_gen<T>(variant: impl Fn(T) -> EmployeeYearMessage) -> impl Fn(T) -> Message{
    move |s| Message::EmployeeYearMessage(variant(s))
}


#[derive(Debug, Clone, ToSql, FromSql, Eq, PartialEq, Copy)]
#[postgres(name="perfomance")]
pub enum Performance {
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
impl Performance {
    const ALL: [Performance; 5] = [
        Performance::Well,
        Performance::Ok,
        Performance::NotWell,
        Performance::SuperPerformer,
        Performance::Manager
    ];
}
impl std::fmt::Display for Performance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Performance::Well => "Well",
                Performance::Ok => "Ok",
                Performance::NotWell => "Not Well",
                Performance::SuperPerformer => "Super Performer",
                Performance::Manager => "Manager"
            }
        )
    }
}

#[derive(Debug, Clone, ToSql, FromSql, Eq, PartialEq, Copy)]
#[postgres(name="salary")]
pub enum SalaryType {
    #[postgres(name="w2")]
    W2,
    #[postgres(name="hourly")]
    Hourly
}
impl Default for SalaryType {
    fn default() -> Self { SalaryType::W2 }
}
impl SalaryType {
    const ALL: [SalaryType; 2] = [
        SalaryType::W2,
        SalaryType::Hourly
    ];
}
impl std::fmt::Display for SalaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SalaryType::W2 => "W2",
                SalaryType::Hourly => "Hourly"
            }
        )
    }
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
    insurance: Option<InsurancePlan>,

    // state time bois!
    e_id_state: text_input::State,
    e_year_state: text_input::State,
    salary_state: text_input::State,
    salary_pick_state: pick_list::State<SalaryType>,
    performance_pick_state: pick_list::State<Performance>,
    amount_state: text_input::State,
    employee_pays_state: text_input::State,
    employer_pays_state: text_input::State,
    ss_create_state: button::State
}

#[derive(Debug, Clone, Default, Copy)]
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

                let social_security = client.query_opt("SELECT * FROM socialSecurity WHERE E_ID = $1 AND e_year = $2;", &[&e_id, &year])
                    .expect("Error getting social security");
                if let Some(ss_row) = social_security {
                    self.social_security = Some(SocialSecurity{
                        amount: ss_row.get("amount"),
                        employee_pays: ss_row.get("employeePays"),
                        employer_pays: ss_row.get("employerPays")
                    })
                } else {
                    self.social_security = None
                }

                let benefits = client.query_opt("SELECT * FROM benefits WHERE E_ID = $1 AND e_year = $2;", &[&e_id, &year])
                    .expect("Error getting social security");
                if let Some(benefit_row) = benefits {
                    self.benefits = Some(Benefits{
                        benefit_type: benefit_row.get("benefitType"),
                        employee_contribution: benefit_row.get("employeeContribution"),
                        employer_contribution: benefit_row.get("employerContribution")
                    })
                } else {
                    self.benefits = None
                }

                let bonus = client.query_opt("SELECT * FROM bonus WHERE E_ID = $1 AND e_year = $2;", &[&e_id, &year])
                    .expect("Error getting social security");
                if let Some(bonus_row) = bonus {
                    self.bonus = Some(Bonus{
                        percentage: bonus_row.get("percentage"),
                        company_sale: bonus_row.get("company_sale")
                    })
                } else {
                    self.bonus = None
                }

                let insurance_plan = client.query_opt("SELECT * FROM insurancePlan WHERE E_ID = $1 AND e_year = $2;", &[&e_id, &year])
                    .expect("Error getting social security");
                if let Some(insurance_row) = insurance_plan {
                    self.insurance = Some(InsurancePlan {
                        insurance_type: insurance_row.get("insuranceType"),
                        premium: insurance_row.get("premium"),
                        employer_contribution: insurance_row.get("employerContribution")
                    })
                } else {
                    self.insurance = None
                }

                info!("{:?}", &self);
                return Some(Message::SelectPage(crate::Page::ViewEmployeeYear))
            }
            EmployeeYearMessage::DummyUpdate(_) => {}
            EmployeeYearMessage::UpdatePerformance(perf) => {
                if (user.usertype == UserType::Administrator) | (user.usertype == UserType::Manager) {
                    self.performance = perf;
                }
            }
            EmployeeYearMessage::UpdateSalaryType(salary_type) => {
                if (user.usertype == UserType::Administrator) | (user.usertype == UserType::Manager) {
                    self.salary_type = salary_type;
                }
            }
            EmployeeYearMessage::UpdateSalary(salary) => {
                if user.usertype == UserType::Administrator {
                    self.salary = salary.parse().unwrap_or(self.salary);
                }
            }
            EmployeeYearMessage::UpdateSSAmount(str) => {
                match &self.social_security {
                    Some(mut ss) => {
                        self.social_security = Some(SocialSecurity{
                            amount: str.parse().unwrap_or(ss.amount),
                            employee_pays: ss.employee_pays,
                            employer_pays: ss.employer_pays
                        });
                    } None => {panic!("Updated SS without having data.")}
                }
            }
            EmployeeYearMessage::UpdateSSEmployeePays(str) => {
                match &self.social_security {
                    Some(mut ss) => {
                        self.social_security = Some(SocialSecurity{
                            amount: ss.amount,
                            employee_pays: str.parse().unwrap_or(ss.employee_pays),
                            employer_pays: ss.employer_pays
                        });
                    } None => {panic!("Updated SS without having data.")}
                }
            }
            EmployeeYearMessage::UpdateSSEmployerPays(str) => {
                match &self.social_security {
                    Some(mut ss) => {
                        self.social_security = Some(SocialSecurity{
                            amount: ss.amount,
                            employee_pays: ss.employee_pays,
                            employer_pays: str.parse().unwrap_or(ss.employer_pays)
                        });
                    } None => {panic!("Updated SS without having data.")}
                }
            }
            EmployeeYearMessage::CreateSS => {
                self.social_security = Some(SocialSecurity{
                    amount: 0.0,
                    employee_pays: 0.0,
                    employer_pays: 0.0
                })
            }
            _ => {}
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(Row::new().push(Text::new("Employee_ID: "))
                .push(text_input::TextInput::new(&mut self.e_id_state, "e_id",
                                                 &*self.e_id.to_string(),
                                                 make_wrapper(EmployeeYearMessage::DummyUpdate))))
            .push(Row::new().push(Text::new("Year: "))
                .push(text_input::TextInput::new(&mut self.e_year_state, "year",
                                                 &*self.e_year.to_string(),
                                                 make_wrapper(EmployeeYearMessage::DummyUpdate))))
            .push(Row::new().push(Text::new("Salary: "))
                .push(text_input::TextInput::new(&mut self.salary_state, "salary",
                                                 &*self.salary.to_string(),
                                                 make_wrapper(EmployeeYearMessage::UpdateSalary))))
            .push(Row::new().push(Text::new("Salary Type:"))
                .push(pick_list::PickList::new(&mut self.salary_pick_state,
                    &SalaryType::ALL[..],
                    Some(self.salary_type),
                    make_wrapper_gen(EmployeeYearMessage::UpdateSalaryType))))
            .push(Row::new().push(Text::new("Performance:"))
                .push(pick_list::PickList::new(&mut self.performance_pick_state,
                    &Performance::ALL[..],
                    Some(self.performance),
                    make_wrapper_gen(EmployeeYearMessage::UpdatePerformance))))
            .push(Row::new().push(Text::new("Social Security:"))
                .push(
                    match &self.social_security {
                        Some(ss_data) => {
                            Column::new()
                                .push(Row::new().push(Text::new("Amount:"))
                                    .push(TextInput::new(&mut self.amount_state,
                                         "Amount",
                                         &*ss_data.amount.to_string(),
                                         make_wrapper(EmployeeYearMessage::UpdateSSAmount))))
                                .push(Row::new().push(Text::new("Employer Pays:"))
                                    .push(TextInput::new(&mut self.employer_pays_state,
                                         "Employer Pays:",
                                         &*ss_data.employer_pays.to_string(),
                                         make_wrapper(EmployeeYearMessage::UpdateSSEmployerPays))))
                                .push(Row::new().push(Text::new("Employee Pays:"))
                                    .push(TextInput::new(&mut self.employee_pays_state,
                                         "Employee Pays:",
                                         &*ss_data.employee_pays.to_string(),
                                         make_wrapper(EmployeeYearMessage::UpdateSSEmployeePays))))
                        } _ => {
                            Column::new().push(Text::new("None on record."))
                                .push(Button::new(&mut self.ss_create_state,
                                                  Text::new("Create SS Record"))
                                    .on_press(Message::EmployeeYearMessage(EmployeeYearMessage::CreateSS)))
                        }
                    }
                ))
            .into()
    }
}
