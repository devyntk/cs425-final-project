use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button;
use log::{info,warn};
use crate::employee::EmployeeMessage::LoadYear;

#[derive(Debug,Clone)]
pub enum EmployeeMessage {
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeEID(String),
    ChangeSSN(String),
    ChangeJobTitle(String),
    ChangeAddress(String),
    LoadEmployee(i32),
    LoadYear(i32),
    SaveChanges
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeMessage(variant(s))
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
    year_states: Vec<button::State>
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
            }
            EmployeeMessage::SaveChanges => {
                client.execute("UPDATE employee SET SSN=$1, firstName=$2, lastName=$3, jobTitle=$4, stateAddress=$5\
                WHERE E_ID = $6;",
                &[&self.ssn, &self.first_name, &self.last_name, &self.job_title, &self.state_address, &self.e_id]);

                //just to cover all of our bases, let's re-load from the DB
                return Some(Message::EmployeeMessage(EmployeeMessage::LoadEmployee(self.e_id)))
            }
            EmployeeMessage::LoadYear(year) => {
                return Some(Message::EmployeeYearMessage(
                    crate::employee_year::EmployeeYearMessage::Load { year, e_id: self.e_id }
                ))
            }
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
            .push(
                button::Button::new(&mut self.save_button, Text::new("Save Changes"))
                    .on_press(Message::EmployeeMessage(EmployeeMessage::SaveChanges))
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
