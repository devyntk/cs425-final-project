use crate::{Message, User, UserType};
use postgres::{Client, Error};
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::{info,warn};
use crate::employee::EmployeeMessage::LoadYear;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum EmployeeMessage {
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeEID(String),
    ChangeSSN(String),
    ChangeJobTitle(String),
    ChangeAddress(String),
    AddDep,
    RemoveDep(i32),
    LoadEmployee(i32),
    LoadYear(i32),
    SaveChanges,
    ChangeDepID(i32, String),
    ChangeDepName(i32, String),
    ChangeDepSSN(i32, String),
    ChangeDepRelation(i32, String),
    ChangeDepBenefit(i32, String),
    ChangeUsername(String),
    ChangePassword(String),
    MakeLogin
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeMessage(variant(s))
}

fn make_dep_wrapper(dep_id: i32, variant: impl Fn(i32, String) -> EmployeeMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeMessage(variant(dep_id, s))
}

#[derive(Debug, Clone, Default)]
pub struct EmployeeState {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    job_title: String,
    state_address: String,
    first_name_state: text_input::State,
    last_name_state: text_input::State,
    e_id_state: text_input::State,
    ssn_state: text_input::State,
    job_title_state: text_input::State,
    address_state: text_input::State,
    save_button: button::State,
    logout_button: button::State,
    years: Vec<i32>,
    year_states: Vec<button::State>,
    dependents: HashMap<i32, Dependent>,
    num_deps: i32,
    add_dep_state: button::State,
    user_emp: Option<UserEmp>,
    make_login_state: button::State
}

#[derive(Debug, Clone, Default)]
pub struct UserEmp {
    username: String,
    password: String,
    user_type: crate::UserType,
    username_state: text_input::State,
    password_state: text_input::State
}
impl UserEmp {
    fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(Row::new().push(Text::new("Username:"))
                .push(TextInput::new(&mut self.username_state, "username",
                    &*self.username,
                    make_wrapper(EmployeeMessage::ChangeUsername)
                )))
            .push(Row::new().push(Text::new("Password:"))
                .push(TextInput::new(&mut self.password_state, "password",
                     &*self.password,
                     make_wrapper(EmployeeMessage::ChangePassword)
                )))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Dependent {
    d_id: i32,
    d_name: String,
    ssn: String,
    relation: String,
    benefits: String,

    d_id_state: text_input::State,
    d_name_state: text_input::State,
    ssn_state: text_input::State,
    relation_state: text_input::State,
    benefits_state: text_input::State,
    delete_state: button::State
}

impl Dependent {
    fn view(&mut self) -> Column<Message> {
        Column::new().push(
            Row::new().push(Text::new("D_ID:"))
                .push(TextInput::new(&mut self.d_id_state, "D_ID",
                                     &*self.d_id.to_string(),
                                     make_dep_wrapper(self.d_id, EmployeeMessage::ChangeDepID)
                ))
        ).push(
            Row::new().push(Text::new("Name:"))
                .push(TextInput::new(&mut self.d_name_state, "Name",
                                     &*self.d_name,
                                     make_dep_wrapper(self.d_id, EmployeeMessage::ChangeDepName)
                ))
        ).push(
            Row::new().push(Text::new("SSN:"))
                .push(TextInput::new(&mut self.ssn_state, "SSN",
                                     &*self.ssn,
                                     make_dep_wrapper(self.d_id, EmployeeMessage::ChangeDepSSN)
                ))
        ).push(
            Row::new().push(Text::new("Relation:"))
                .push(TextInput::new(&mut self.relation_state, "Relation",
                                     &*self.relation,
                                     make_dep_wrapper(self.d_id, EmployeeMessage::ChangeDepRelation)
                ))
        ).push(
            Row::new().push(Text::new("Benefit:"))
                .push(TextInput::new(&mut self.benefits_state, "Benefit",
                                     &*self.benefits,
                                     make_dep_wrapper(self.d_id, EmployeeMessage::ChangeDepBenefit)
                ))
        ).push(
            Button::new(&mut self.delete_state, Text::new("Delete Dependent"))
                .on_press(Message::EmployeeMessage(EmployeeMessage::RemoveDep(self.d_id)))
        )
    }
}

impl EmployeeState {
    pub fn new() -> Self {
        EmployeeState::default()
    }

    pub(crate) fn update(&mut self, msg: EmployeeMessage, client: &mut Client, user: &User) -> Option<Message> {
        match msg {
            EmployeeMessage::ChangeFirstName(str) => {
                self.first_name = str;
            }
            EmployeeMessage::ChangeLastName(str) => {
                self.last_name = str;
            }
            EmployeeMessage::ChangeEID(str) => {
                warn!("You should not be modifying the E_ID!")
                // if user.usertype == UserType::Administrator {
                //     self.e_id = str.parse().expect("Cannot parse Employee ID!");
                // }
            }
            EmployeeMessage::ChangeSSN(str) => {
                self.ssn = str;
            }
            EmployeeMessage::ChangeJobTitle(str) => {
                match user.usertype {
                    UserType::Administrator | UserType::Manager => {
                        self.job_title = str;
                    }
                    UserType::Employee => {}
                }
            }
            EmployeeMessage::ChangeAddress(str) => {
                self.state_address = str;
            }
            EmployeeMessage::LoadEmployee(e_id) => {
                let employee = client.query_one("SELECT * FROM employee WHERE E_ID = $1;", &[&e_id])
                    .expect("Can't find employee!");
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.job_title = employee.get("jobTitle");
                self.state_address = employee.get("stateAddress");

                let years = client.query("SELECT e_year FROM employeeYear WHERE e_id = $1;", &[&e_id])
                    .expect("Cannot Find Years");
                self.years = Vec::new();
                self.year_states = Vec::new();
                for year in years {
                    self.years.push(year.get("e_year"));
                    self.year_states.push(button::State::new());
                };

                let dependents = client.query("SELECT * FROM dependent WHERE e_id = $1;", &[&e_id])
                    .expect("Cannot Find dependent(s)");
                self.dependents = HashMap::new();
                self.num_deps = 0;
                for dependent in dependents {
                    self.dependents.insert(dependent.get("d_id"),
                    Dependent{
                        d_id: dependent.get("d_id"),
                        d_name: dependent.get("d_name"),
                        ssn: dependent.get("SSN"),
                        relation: dependent.get("relation"),
                        benefits: dependent.get("benefits"),
                        d_id_state: text_input::State::default(),
                        d_name_state: text_input::State::default(),
                        ssn_state: text_input::State::default(),
                        relation_state: text_input::State::default(),
                        benefits_state: text_input::State::default(),
                        delete_state: button::State::default()
                    });
                    let user = client.query_opt("SELECT * FROM user_tbl WHERE e_id = $1;",
                                                &[&self.e_id]).expect("Error getting user");
                    match user {
                        None => {
                            self.user_emp = None;
                        } Some(user_row) => {
                            self.user_emp = Some(UserEmp{
                                username: user_row.get("username"),
                                password: user_row.get("psswrd"),
                                user_type: user_row.get("user_type"),
                                username_state: Default::default(),
                                password_state: Default::default()
                            })
                        }
                    }

                    self.num_deps += 1;

                };
            }
            EmployeeMessage::SaveChanges => {
                client.execute("UPDATE employee SET SSN=$1, firstName=$2, lastName=$3, jobTitle=$4, stateAddress=$5\
                WHERE E_ID = $6;",
                &[&self.ssn, &self.first_name, &self.last_name, &self.job_title, &self.state_address, &self.e_id]);

                for dep in self.dependents.values() {
                    client.execute("INSERT INTO dependent (D_ID, E_ID, d_name, SSN, relation, benefits) \
                    VALUES ($1, $2, $3, $4, $5, $6) \
                    ON CONFLICT (D_ID, E_ID) DO UPDATE \
                        SET d_name = $3, \
                        SSN = $4, \
                        relation = $5, \
                        benefits = $6",
                                   &[&dep.d_id, &self.e_id, &dep.d_name, &dep.ssn, &dep.relation, &dep.benefits])
                        .expect("Cannot update dependent");
                }
                if let Some(user) = &self.user_emp {
                    client.execute("INSERT INTO user_tbl (username, psswrd, user_type, E_ID) \
                    VALUES ($1, $2, $3, $4) \
                    ON CONFLICT (E_ID) DO UPDATE \
                        SET username = $1, \
                        psswrd = $2, \
                        user_type = $3",
                                   &[&user.username, &user.password, &user.user_type, &self.e_id])
                        .expect("Cannot update dependent");

                }

                //just to cover all of our bases, let's re-load from the DB
                return Some(Message::EmployeeMessage(EmployeeMessage::LoadEmployee(self.e_id)))
            }
            EmployeeMessage::LoadYear(year) => {
                return Some(Message::EmployeeYearMessage(
                    crate::employee_year::EmployeeYearMessage::Load { year, e_id: self.e_id }
                ))
            }
            EmployeeMessage::AddDep => {
                self.dependents.insert(self.num_deps+1,
                   Dependent{
                       d_id: self.num_deps+1,
                       d_name: "Dependent Name".to_string(),
                       ssn: "SSN".to_string(),
                       relation: "Relation".to_string(),
                       benefits: "Benefits".to_string(),
                       d_id_state: text_input::State::default(),
                       d_name_state: text_input::State::default(),
                       ssn_state: text_input::State::default(),
                       relation_state: text_input::State::default(),
                       benefits_state: text_input::State::default(),
                       delete_state: button::State::default()
                   });
                self.num_deps += 1;
            }
            EmployeeMessage::RemoveDep(idx) => {
                match self.dependents.get(&idx) {
                    None => {panic!("Trying to delete unknown idx")}
                    Some(dep) => {
                        match client.execute("DELETE FROM dependent WHERE e_id=$1 AND d_id=$2;",
                        &[&self.e_id, &idx]) {
                            Ok(_) => {}
                            Err(err) => {
                                warn!("Failure deleting dependent: {:?}", err)
                            }
                        }
                        self.dependents.remove(&idx);
                    }
                }
            }
            EmployeeMessage::ChangeDepName(idx, str) => {
                match self.dependents.get_mut(&idx) {
                    None => { panic!("Cannot find Dep IDX")}
                    Some(dep) => {
                        dep.d_name = str;
                    }
                }
            }
            EmployeeMessage::ChangeDepBenefit(idx, str) => {
                match self.dependents.get_mut(&idx) {
                    None => { panic!("Cannot find Dep IDX")}
                    Some(dep) => {
                        dep.benefits = str;
                    }
                }
            }
            EmployeeMessage::ChangeDepRelation(idx, str) => {
                match self.dependents.get_mut(&idx) {
                    None => { panic!("Cannot find Dep IDX")}
                    Some(dep) => {
                        dep.relation = str;
                    }
                }
            }
            EmployeeMessage::ChangeDepSSN(idx, str) => {
                match self.dependents.get_mut(&idx) {
                    None => { panic!("Cannot find Dep IDX")}
                    Some(dep) => {
                        dep.ssn = str;
                    }
                }
            }
            EmployeeMessage::ChangeUsername(str) => {
                match &mut self.user_emp {
                    None => {panic!("No user on record")}
                    Some(user) => {
                       user.username = str
                    }
                }
            }
            EmployeeMessage::ChangePassword(str) => {
                match &mut self.user_emp {
                    None => {panic!("No user on record")}
                    Some(user) => {
                        user.password = str
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        let mut column: Column<Message> = Column::new()
            .push( Row::new()
                .push(Text::new("Employee ID:"))
                .push(TextInput::new(
                    &mut self.e_id_state,
                    "E_ID",
                    self.e_id.to_string().as_str(),
                    make_wrapper(EmployeeMessage::ChangeEID))
                )
            )
            .push( Row::new()
                .push(Text::new("First Name:"))
                .push(TextInput::new(
                    &mut self.first_name_state,
                    "first name",
                    &*self.first_name,
                    make_wrapper(EmployeeMessage::ChangeFirstName))
                )
            )
            .push( Row::new()
                .push(Text::new("Last Name:"))
                .push(TextInput::new(
                    &mut self.last_name_state,
                    "last name",
                    &*self.last_name,
                    make_wrapper(EmployeeMessage::ChangeLastName))
                )
            )
            .push( Row::new()
                .push(Text::new("SSN:"))
                .push(TextInput::new(
                    &mut self.ssn_state,
                    "ssn",
                    &*self.ssn,
                    make_wrapper(EmployeeMessage::ChangeSSN))
                )
            )
            .push( Row::new()
                .push(Text::new("Job Title:"))
                .push(TextInput::new(
                    &mut self.job_title_state,
                    "job title",
                    &*self.job_title,
                    make_wrapper(EmployeeMessage::ChangeJobTitle))
                )
            )
            .push( Row::new()
                .push(Text::new("Address:"))
                .push(TextInput::new(
                    &mut self.address_state,
                    "Address",
                    &*self.state_address,
                    make_wrapper(EmployeeMessage::ChangeAddress))
                )
            )
            .push(match !self.years.is_empty() {
                true => {
                    let mut year_row: Row<Message> = Row::new()
                        .push(Text::new("Years on Record:"));

                    for (i, state) in self.year_states.iter_mut().enumerate() {
                        year_row = year_row.push(Button::new(state, Text::new(self.years[i].to_string()))
                            .on_press(Message::EmployeeMessage(LoadYear(self.years[i] as i32))));
                    }
                    year_row
                }
                false => {Row::new().push(Text::new("No associated Years found."))}
            })
            .push(Row::new().push(Text::new("Dependents:"))
                .push(self.dependents.iter_mut().fold(
                    Column::new(),
                    |parent: Column<Message>, (d_id, dep)| {
                        parent.push(dep.view())
                    }
                ).push(Button::new(&mut self.add_dep_state, Text::new("Add Dependent"))
                    .on_press(Message::EmployeeMessage(EmployeeMessage::AddDep)))
            ))
            .push(match &mut self.user_emp {
                None => {
                    Column::new().push(Button::new(&mut self.make_login_state, Text::new("Make Login"))
                        .on_press(Message::EmployeeMessage(EmployeeMessage::MakeLogin)))
                }
                Some(login_emp) => {
                    login_emp.view()
                }
            })
            .push(
                button::Button::new(&mut self.save_button, Text::new("Save Changes"))
                    .on_press(Message::EmployeeMessage(EmployeeMessage::SaveChanges))
            )
            .push(match user.usertype {
                UserType::Employee => {
                    Button::new(&mut self.logout_button, Text::new("Log Out"))
                        .on_press(Message::LogOut)
                }
                _ => {
                    Button::new(&mut self.logout_button, Text::new("Back to Menu"))
                        .on_press(Message::SelectPage(crate::Page::Main))

                }
            });

        column.into()
    }
}
