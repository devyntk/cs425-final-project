use crate::{Message, User, UserType};
use iced::button;
use iced::{pick_list, text_input, Button, Column, Element, Row, Text, TextInput};
use log::{info, warn};
use postgres::Client;
use postgres_types::{FromSql, ToSql};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum EmployeeYearMessage {
    Load { year: i32, e_id: i32 },
    DummyUpdate(String),
    UpdateSalaryType(SalaryType),
    UpdatePerformance(Performance),
    UpdateSalary(String),
    Save,
    Back,
    ViewW2,
    ViewPaycheck,

    UpdateSSAmount(String),
    UpdateSSEmployeePays(String),
    UpdateSSEmployerPays(String),
    CreateSS,

    UpdateBenefitType(String),
    UpdateBenefitEmployee(String),
    UpdateBenefitEmployer(String),
    CreateBenefit,

    UpdateBonusPercentage(String),
    UpdateBonusSale(String),
    CreateBonus,

    UpdateInsuranceType(String),
    UpdateInsurancePremium(String),
    UpdateInsuranceEmployer(String),
    CreateInsurance,
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeYearMessage) -> impl Fn(String) -> Message {
    move |s| Message::EmployeeYearMessage(variant(s))
}
fn make_wrapper_gen<T>(variant: impl Fn(T) -> EmployeeYearMessage) -> impl Fn(T) -> Message {
    move |s| Message::EmployeeYearMessage(variant(s))
}

#[derive(Debug, Clone, ToSql, FromSql, Eq, PartialEq, Copy)]
#[postgres(name = "perfomance")]
pub enum Performance {
    #[postgres(name = "well")]
    Well,
    #[postgres(name = "ok")]
    Ok,
    #[postgres(name = "not_well")]
    NotWell,
    #[postgres(name = "super_performer")]
    SuperPerformer,
    #[postgres(name = "manager")]
    Manager,
}
impl Default for Performance {
    fn default() -> Self {
        Performance::Ok
    }
}
impl Performance {
    const ALL: [Performance; 5] = [
        Performance::Well,
        Performance::Ok,
        Performance::NotWell,
        Performance::SuperPerformer,
        Performance::Manager,
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
                Performance::Manager => "Manager",
            }
        )
    }
}

#[derive(Debug, Clone, ToSql, FromSql, Eq, PartialEq, Copy)]
#[postgres(name = "salary")]
pub enum SalaryType {
    #[postgres(name = "w2")]
    W2,
    #[postgres(name = "hourly")]
    Hourly,
}
impl Default for SalaryType {
    fn default() -> Self {
        SalaryType::W2
    }
}
impl SalaryType {
    const ALL: [SalaryType; 2] = [SalaryType::W2, SalaryType::Hourly];
}
impl std::fmt::Display for SalaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SalaryType::W2 => "W2",
                SalaryType::Hourly => "Hourly",
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
    save_state: button::State,
    back_state: button::State,
    w2_state: button::State,
    paycheck_state: button::State,

    // social security state
    amount_state: text_input::State,
    employee_pays_state: text_input::State,
    employer_pays_state: text_input::State,
    ss_create_state: button::State,

    //benefits state
    benefit_type_state: text_input::State,
    benefit_employee_contribution_state: text_input::State,
    benefit_employer_contribution_state: text_input::State,
    benefit_create_state: button::State,

    // bonus state
    bonus_percentage_state: text_input::State,
    bonus_sale_state: text_input::State,
    bonus_create_state: button::State,

    // insurance state
    insurance_type_state: text_input::State,
    insurance_premium_state: text_input::State,
    insurance_employer_state: text_input::State,
    insurance_create_state: button::State,
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
    employer_contribution: f32,
}

#[derive(Debug, Clone, Default)]
struct Bonus {
    percentage: f32,
    company_sale: f32,
}

#[derive(Debug, Clone, Default)]
struct InsurancePlan {
    insurance_type: String,
    premium: f32,
    employer_contribution: f32,
}

impl EmployeeYearState {
    pub fn new() -> Self {
        EmployeeYearState::default()
    }

    pub(crate) fn update(
        &mut self,
        msg: EmployeeYearMessage,
        client: &mut Client,
        user: &User,
    ) -> Option<Message> {
        match msg {
            EmployeeYearMessage::Load { year, e_id } => {
                let employeeyr = client
                    .query_one(
                        "SELECT * FROM employeeYear WHERE E_ID = $1 AND e_year = $2;",
                        &[&e_id, &year],
                    )
                    .expect("Can't find employee!");
                self.e_id = employeeyr.get("E_ID");
                self.e_year = employeeyr.get("e_year");
                self.salary = employeeyr.get("salary");
                self.salary_type = employeeyr.get("salaryType");
                self.performance = employeeyr.get("performance");

                let social_security = client
                    .query_opt(
                        "SELECT * FROM socialSecurity WHERE E_ID = $1 AND e_year = $2;",
                        &[&e_id, &year],
                    )
                    .expect("Error getting social security");
                if let Some(ss_row) = social_security {
                    self.social_security = Some(SocialSecurity {
                        amount: ss_row.get("amount"),
                        employee_pays: ss_row.get("employeePays"),
                        employer_pays: ss_row.get("employerPays"),
                    })
                } else {
                    self.social_security = None
                }

                let benefits = client
                    .query_opt(
                        "SELECT * FROM benefits WHERE E_ID = $1 AND e_year = $2;",
                        &[&e_id, &year],
                    )
                    .expect("Error getting social security");
                if let Some(benefit_row) = benefits {
                    self.benefits = Some(Benefits {
                        benefit_type: benefit_row.get("benefitType"),
                        employee_contribution: benefit_row.get("employeeContribution"),
                        employer_contribution: benefit_row.get("employerContribution"),
                    })
                } else {
                    self.benefits = None
                }

                let bonus = client
                    .query_opt(
                        "SELECT * FROM bonus WHERE E_ID = $1 AND e_year = $2;",
                        &[&e_id, &year],
                    )
                    .expect("Error getting social security");
                if let Some(bonus_row) = bonus {
                    self.bonus = Some(Bonus {
                        percentage: bonus_row.get("percentage"),
                        company_sale: bonus_row.get("company_sale"),
                    })
                } else {
                    self.bonus = None
                }

                let insurance_plan = client
                    .query_opt(
                        "SELECT * FROM insurancePlan WHERE E_ID = $1 AND e_year = $2;",
                        &[&e_id, &year],
                    )
                    .expect("Error getting social security");
                if let Some(insurance_row) = insurance_plan {
                    self.insurance = Some(InsurancePlan {
                        insurance_type: insurance_row.get("insuranceType"),
                        premium: insurance_row.get("premium"),
                        employer_contribution: insurance_row.get("employerContribution"),
                    })
                } else {
                    self.insurance = None
                }

                info!("{:?}", &self);
                return Some(Message::SelectPage(crate::Page::ViewEmployeeYear));
            }
            EmployeeYearMessage::Save => {
                client.execute(
                    "UPDATE employeeYear SET salary=$3, salaryType=$4, performance=$5\
                WHERE E_ID = $1 AND e_year=$2;",
                    &[
                        &self.e_id,
                        &self.e_year,
                        &self.salary,
                        &self.salary_type,
                        &self.performance,
                    ],
                );
                if let Some(ss) = &self.social_security {
                    client.execute("INSERT INTO socialSecurity (E_ID, e_year, amount, employeePays, employerPays)\
                    VALUES ($1, $2, $3, $4, $5) \
                    ON CONFLICT (E_ID, e_year) DO UPDATE \
                        SET amount = $3,\
                        employeePays = $4, \
                        employerPays = $5",
                                   &[&self.e_id, &self.e_year, &ss.amount, &ss.employee_pays, &ss.employer_pays])
                        .expect("Cannot update social Security");
                }

                if let Some(benefit) = &self.benefits {
                    client.execute("INSERT INTO benefits (E_ID, e_year, benefitType, employeeContribution, employerContribution)\
                    VALUES ($1, $2, $3, $4, $5) \
                    ON CONFLICT (E_ID, e_year) DO UPDATE \
                        SET benefitType = $3,\
                        employeeContribution = $4, \
                        employerContribution = $5",
                                   &[&self.e_id, &self.e_year, &benefit.benefit_type, &benefit.employee_contribution, &benefit.employer_contribution])
                        .expect("Cannot update benefit");
                }

                if let Some(bonus) = &self.bonus {
                    client
                        .execute(
                            "INSERT INTO bonus (E_ID, e_year, percentage, company_sale) \
                    VALUES ($1, $2, $3, $4) \
                    ON CONFLICT (E_ID, e_year) DO UPDATE \
                        SET percentage = $3, \
                        company_sale = $4;",
                            &[
                                &self.e_id,
                                &self.e_year,
                                &bonus.percentage,
                                &bonus.company_sale,
                            ],
                        )
                        .expect("Cannot update bonus");
                }

                if let Some(insurance) = &self.insurance {
                    client.execute("INSERT INTO insurancePlan (E_ID, e_year, insuranceType, premium, employerContribution)\
                    VALUES ($1, $2, $3, $4, $5) \
                    ON CONFLICT (E_ID, e_year) DO UPDATE \
                        SET insuranceType = $3,\
                        premium = $4, \
                        employerContribution = $5",
                                   &[&self.e_id, &self.e_year, &insurance.insurance_type, &insurance.premium, &insurance.employer_contribution])
                        .expect("Cannot update insurance");
                }

                return Some(Message::EmployeeYearMessage(EmployeeYearMessage::Load {
                    year: self.e_year,
                    e_id: self.e_id,
                }));
            }
            EmployeeYearMessage::Back => {
                return Some(Message::SelectPage(crate::Page::ViewEmployee))
            }
            EmployeeYearMessage::DummyUpdate(_) => {}
            EmployeeYearMessage::UpdatePerformance(perf) => {
                if (user.usertype == UserType::Administrator) | (user.usertype == UserType::Manager)
                {
                    self.performance = perf;
                }
            }
            EmployeeYearMessage::UpdateSalaryType(salary_type) => {
                if (user.usertype == UserType::Administrator) | (user.usertype == UserType::Manager)
                {
                    self.salary_type = salary_type;
                }
            }
            EmployeeYearMessage::UpdateSalary(salary) => {
                if user.usertype == UserType::Administrator {
                    self.salary = salary.parse().unwrap_or(self.salary);
                }
            }
            EmployeeYearMessage::UpdateSSAmount(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.social_security {
                    Some(mut ss) => {
                        self.social_security = Some(SocialSecurity {
                            amount: str.parse().unwrap_or(ss.amount),
                            employee_pays: ss.employee_pays,
                            employer_pays: ss.employer_pays,
                        });
                    }
                    None => {
                        panic!("Updated SS without having data.")
                    }
                }
            }
            EmployeeYearMessage::UpdateSSEmployeePays(str) => match &self.social_security {
                Some(mut ss) => {
                    self.social_security = Some(SocialSecurity {
                        amount: ss.amount,
                        employee_pays: str.parse().unwrap_or(ss.employee_pays),
                        employer_pays: ss.employer_pays,
                    });
                }
                None => {
                    panic!("Updated SS without having data.")
                }
            },
            EmployeeYearMessage::UpdateSSEmployerPays(str) => match &self.social_security {
                Some(mut ss) => {
                    self.social_security = Some(SocialSecurity {
                        amount: ss.amount,
                        employee_pays: ss.employee_pays,
                        employer_pays: str.parse().unwrap_or(ss.employer_pays),
                    });
                }
                None => {
                    panic!("Updated SS without having data.")
                }
            },
            EmployeeYearMessage::CreateSS => {
                if !user.is_manager() {
                    return None;
                }
                self.social_security = Some(SocialSecurity {
                    amount: 0.0,
                    employee_pays: 0.0,
                    employer_pays: 0.0,
                })
            }
            EmployeeYearMessage::UpdateBenefitType(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.benefits {
                    Some(benefits) => {
                        self.benefits = Some(Benefits {
                            benefit_type: str,
                            employee_contribution: benefits.employee_contribution,
                            employer_contribution: benefits.employer_contribution,
                        });
                    }
                    None => {
                        panic!("Updated Benefits without having data.")
                    }
                }
            }
            EmployeeYearMessage::UpdateBenefitEmployee(str) => match &self.benefits {
                Some(benefits) => {
                    self.benefits = Some(Benefits {
                        benefit_type: benefits.benefit_type.clone(),
                        employee_contribution: str
                            .parse()
                            .unwrap_or(benefits.employee_contribution),
                        employer_contribution: benefits.employer_contribution,
                    });
                }
                None => {
                    panic!("Updated Benefits without having data.")
                }
            },
            EmployeeYearMessage::UpdateBenefitEmployer(str) => match &self.benefits {
                Some(benefits) => {
                    self.benefits = Some(Benefits {
                        benefit_type: benefits.benefit_type.clone(),
                        employee_contribution: benefits.employee_contribution,
                        employer_contribution: str
                            .parse()
                            .unwrap_or(benefits.employer_contribution),
                    });
                }
                None => {
                    panic!("Updated Benefits without having data.")
                }
            },
            EmployeeYearMessage::CreateBenefit => {
                if !user.is_manager() {
                    return None;
                }
                self.benefits = Some(Benefits {
                    benefit_type: "Insert Here".parse().unwrap(),
                    employee_contribution: 0.0,
                    employer_contribution: 0.0,
                })
            }
            EmployeeYearMessage::UpdateBonusPercentage(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.bonus {
                    Some(bonus) => {
                        self.bonus = Some(Bonus {
                            percentage: str.parse().unwrap_or(bonus.percentage),
                            company_sale: bonus.company_sale,
                        });
                    }
                    None => {
                        panic!("Updated Benefits without having data.")
                    }
                }
            }
            EmployeeYearMessage::UpdateBonusSale(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.bonus {
                    Some(bonus) => {
                        self.bonus = Some(Bonus {
                            percentage: bonus.percentage,
                            company_sale: str.parse().unwrap_or(bonus.company_sale),
                        });
                    }
                    None => {
                        panic!("Updated Benefits without having data.")
                    }
                }
            }
            EmployeeYearMessage::CreateBonus => {
                if !user.is_manager() {
                    return None;
                }
                self.bonus = Some(Bonus {
                    percentage: 0.0,
                    company_sale: 0.0,
                })
            }
            EmployeeYearMessage::UpdateInsuranceType(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.insurance {
                    Some(insurance) => {
                        self.insurance = Some(InsurancePlan {
                            insurance_type: str,
                            premium: insurance.premium,
                            employer_contribution: insurance.employer_contribution,
                        });
                    }
                    None => {
                        panic!("Updated Benefits without having data.")
                    }
                }
            }
            EmployeeYearMessage::UpdateInsuranceEmployer(str) => match &self.insurance {
                Some(insurance) => {
                    self.insurance = Some(InsurancePlan {
                        insurance_type: insurance.insurance_type.clone(),
                        premium: insurance.premium,
                        employer_contribution: str
                            .parse()
                            .unwrap_or(insurance.employer_contribution),
                    });
                }
                None => {
                    panic!("Updated Benefits without having data.")
                }
            },
            EmployeeYearMessage::UpdateInsurancePremium(str) => {
                if !user.is_manager() {
                    return None;
                }
                match &self.insurance {
                    Some(insurance) => {
                        self.insurance = Some(InsurancePlan {
                            insurance_type: insurance.insurance_type.clone(),
                            premium: str.parse().unwrap_or(insurance.premium),
                            employer_contribution: insurance.employer_contribution,
                        });
                    }
                    None => {
                        panic!("Updated Benefits without having data.")
                    }
                }
            }
            EmployeeYearMessage::CreateInsurance => {
                self.insurance = Some(InsurancePlan {
                    insurance_type: "".to_string(),
                    premium: 0.0,
                    employer_contribution: 0.0,
                })
            }
            EmployeeYearMessage::ViewPaycheck => {
                return Some(Message::PaycheckMessage(
                    crate::paycheck::PaycheckMessage::Load {
                        year: self.e_year,
                        e_id: self.e_id,
                    },
                ))
            }
            EmployeeYearMessage::ViewW2 => {
                return Some(Message::W2Message(crate::w2::W2Message::W2Report {
                    year: self.e_year,
                    e_id: self.e_id,
                }))
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(Text::new("Employee_ID: "))
                    .push(text_input::TextInput::new(
                        &mut self.e_id_state,
                        "e_id",
                        &*self.e_id.to_string(),
                        make_wrapper(EmployeeYearMessage::DummyUpdate),
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Year: "))
                    .push(text_input::TextInput::new(
                        &mut self.e_year_state,
                        "year",
                        &*self.e_year.to_string(),
                        make_wrapper(EmployeeYearMessage::DummyUpdate),
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Salary: "))
                    .push(text_input::TextInput::new(
                        &mut self.salary_state,
                        "salary",
                        &*self.salary.to_string(),
                        make_wrapper(EmployeeYearMessage::UpdateSalary),
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Salary Type:"))
                    .push(pick_list::PickList::new(
                        &mut self.salary_pick_state,
                        &SalaryType::ALL[..],
                        Some(self.salary_type),
                        make_wrapper_gen(EmployeeYearMessage::UpdateSalaryType),
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Performance:"))
                    .push(pick_list::PickList::new(
                        &mut self.performance_pick_state,
                        &Performance::ALL[..],
                        Some(self.performance),
                        make_wrapper_gen(EmployeeYearMessage::UpdatePerformance),
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Social Security:"))
                    .push(match &self.social_security {
                        Some(ss_data) => Column::new()
                            .push(Row::new().push(Text::new("Amount:")).push(TextInput::new(
                                &mut self.amount_state,
                                "Amount",
                                &*ss_data.amount.to_string(),
                                make_wrapper(EmployeeYearMessage::UpdateSSAmount),
                            )))
                            .push(Row::new().push(Text::new("Employer Pays:")).push(
                                TextInput::new(
                                    &mut self.employer_pays_state,
                                    "Employer Pays:",
                                    &*ss_data.employer_pays.to_string(),
                                    make_wrapper(EmployeeYearMessage::UpdateSSEmployerPays),
                                ),
                            ))
                            .push(Row::new().push(Text::new("Employee Pays:")).push(
                                TextInput::new(
                                    &mut self.employee_pays_state,
                                    "Employee Pays:",
                                    &*ss_data.employee_pays.to_string(),
                                    make_wrapper(EmployeeYearMessage::UpdateSSEmployeePays),
                                ),
                            )),
                        _ => Column::new().push(Text::new("None on record.")).push(
                            Button::new(&mut self.ss_create_state, Text::new("Create SS Record"))
                                .on_press(Message::EmployeeYearMessage(
                                    EmployeeYearMessage::CreateSS,
                                )),
                        ),
                    }),
            )
            .push(
                Row::new()
                    .push(Text::new("Benefit:"))
                    .push(match &self.benefits {
                        Some(benefits) => Column::new()
                            .push(Row::new().push(Text::new("Type:")).push(TextInput::new(
                                &mut self.benefit_type_state,
                                "type",
                                &*benefits.benefit_type,
                                make_wrapper(EmployeeYearMessage::UpdateBenefitType),
                            )))
                            .push(Row::new().push(Text::new("Employer Contribution:")).push(
                                TextInput::new(
                                    &mut self.benefit_employer_contribution_state,
                                    "Employer Pays:",
                                    &*benefits.employer_contribution.to_string(),
                                    make_wrapper(EmployeeYearMessage::UpdateBenefitEmployer),
                                ),
                            ))
                            .push(Row::new().push(Text::new("Employee Contribution:")).push(
                                TextInput::new(
                                    &mut self.benefit_employee_contribution_state,
                                    "Employee Pays:",
                                    &*benefits.employee_contribution.to_string(),
                                    make_wrapper(EmployeeYearMessage::UpdateSSEmployeePays),
                                ),
                            )),
                        _ => Column::new().push(Text::new("None on record.")).push(
                            Button::new(
                                &mut self.benefit_create_state,
                                Text::new("Create Benefit"),
                            )
                            .on_press(Message::EmployeeYearMessage(
                                EmployeeYearMessage::CreateBenefit,
                            )),
                        ),
                    }),
            )
            .push(
                Row::new()
                    .push(Text::new("Bonus:"))
                    .push(match &self.bonus {
                        Some(bonus) => {
                            Column::new()
                                .push(Row::new().push(Text::new("Percentage:")).push(
                                    TextInput::new(
                                        &mut self.bonus_percentage_state,
                                        "Percentage",
                                        &*bonus.percentage.to_string(),
                                        make_wrapper(EmployeeYearMessage::UpdateBonusPercentage),
                                    ),
                                ))
                                .push(Row::new().push(Text::new("Company Sale:")).push(
                                    TextInput::new(
                                        &mut self.bonus_sale_state,
                                        "Company Sale",
                                        &*bonus.company_sale.to_string(),
                                        make_wrapper(EmployeeYearMessage::UpdateBonusSale),
                                    ),
                                ))
                        }
                        _ => Column::new().push(Text::new("None on record.")).push(
                            Button::new(&mut self.bonus_create_state, Text::new("Create Benefit"))
                                .on_press(Message::EmployeeYearMessage(
                                    EmployeeYearMessage::CreateBonus,
                                )),
                        ),
                    }),
            )
            .push(
                Row::new()
                    .push(Text::new("Insurance:"))
                    .push(match &self.insurance {
                        Some(insurance) => Column::new()
                            .push(Row::new().push(Text::new("Type:")).push(TextInput::new(
                                &mut self.insurance_type_state,
                                "type",
                                &*insurance.insurance_type,
                                make_wrapper(EmployeeYearMessage::UpdateInsuranceType),
                            )))
                            .push(Row::new().push(Text::new("Premium:")).push(TextInput::new(
                                &mut self.insurance_premium_state,
                                "premium",
                                &*insurance.premium.to_string(),
                                make_wrapper(EmployeeYearMessage::UpdateInsurancePremium),
                            )))
                            .push(Row::new().push(Text::new("Employer Contribution:")).push(
                                TextInput::new(
                                    &mut self.insurance_employer_state,
                                    "employer contribution",
                                    &*insurance.employer_contribution.to_string(),
                                    make_wrapper(EmployeeYearMessage::UpdateInsuranceEmployer),
                                ),
                            )),
                        _ => Column::new().push(Text::new("None on record.")).push(
                            Button::new(
                                &mut self.insurance_create_state,
                                Text::new("Create Insurance"),
                            )
                            .on_press(Message::EmployeeYearMessage(
                                EmployeeYearMessage::CreateInsurance,
                            )),
                        ),
                    }),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.w2_state, Text::new("View W2"))
                            .on_press(Message::EmployeeYearMessage(EmployeeYearMessage::ViewW2)),
                    )
                    .push(
                        Button::new(&mut self.paycheck_state, Text::new("View Paycheck")).on_press(
                            Message::EmployeeYearMessage(EmployeeYearMessage::ViewPaycheck),
                        ),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.back_state, Text::new("Back to Employee"))
                            .on_press(Message::EmployeeYearMessage(EmployeeYearMessage::Back)),
                    )
                    .push(
                        Button::new(&mut self.save_state, Text::new("Update Employee"))
                            .on_press(Message::EmployeeYearMessage(EmployeeYearMessage::Save)),
                    ),
            )
            .into()
    }
}
