use crate::{Message, User, UserType};
use postgres::Client;
use iced::{Column, Element, Text, Button, Row, TextInput, text_input};
use iced::button::State;

#[derive(Debug,Clone)]
pub enum EmployeeMessage {
    ChangeFirstName(String),
    LoadEmployee(i32)
}
fn make_wrapper(variant: impl Fn(String) -> EmployeeMessage) -> impl Fn(String) -> Message{
    move |s| Message::EmployeeMessage(variant(s))
}

#[derive(Debug, Clone)]
pub struct EmployeeState {
    e_id: i32,
    ssn: String,
    first_name: String,
    last_name: String,
    job_title: String,
    state_address: String,
    first_name_state: text_input::State
}

impl EmployeeState {
    pub fn new() -> Self {
        EmployeeState {
            e_id: 0,
            ssn: "".parse().unwrap(),
            first_name: "".parse().unwrap(),
            last_name: "".parse().unwrap(),
            job_title: "".parse().unwrap(),
            state_address: "".parse().unwrap(),
            first_name_state: text_input::State::default()
        }
    }

    pub(crate) fn update(&mut self, msg: EmployeeMessage, client: &mut Client) -> Option<Message> {
        match msg {
            EmployeeMessage::ChangeFirstName(str) => {
                self.first_name = str;
            }
            EmployeeMessage::LoadEmployee(e_id) => {
                let employee =client.query_one("SELECT * FROM employee WHERE E_ID = $1", &[&e_id]).expect("Can't find employee!");
                self.e_id = employee.get("E_ID");
                self.ssn = employee.get("SSN");
                self.first_name = employee.get("firstName");
                self.last_name = employee.get("lastName");
                self.job_title = employee.get("jobTitle");
                self.state_address = employee.get("stateAddress");
            }
        }
        None
    }

    pub(crate) fn view(&mut self, user: &User) -> Element<Message> {
        Column::new()
            .push( Row::new()
                .push(Text::new("First Name:"))
                .push(TextInput::new(&mut self.first_name_state, "first name", &*self.first_name, make_wrapper(EmployeeMessage::ChangeFirstName)))
            )
            .into()
    }
}
