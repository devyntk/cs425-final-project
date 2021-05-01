use emp_database;


create table User(
	username varchar(32) not null unique,
	psswrd varchar (64) not null,
	IsAdmin bit not null,
	IsEmployee bit not null,
	IsEmployer bit not null,
	HasDependent bit not null,
	user_ID INT not null AUTO_INCREMENT=1,
	primary key(user_ID)
);

insert into employee 
(E_ID, SSN, firstName, lastName, jobTitle, stateAddress)
values
(100, 123456789, 'Paige', 'Turner', 'Manager', 'Illinois' ),
(101, 123456788, 'Elon' 'Musk', 'Salesperson', 'Ohio'),
(102, 123456777, 'May', 'Day', 'Salesperson', 'Florida'),
(103, 123456666, 'Al', 'Dente', 'Salesperson', 'Illinois'),
(104, 123455555, 'Devyn', 'Keeney', 'Robotics_consultant', 'Kansas'),
(105, 123444444, 'Khushboo', 'Bhattu', 'Agent_of_Chaos', 'Ohio');

insert into socialSecurity
(E_ID, SSN, amount, employeepays, employerPays)
values
(100, 123456789, 5000, 5000, 0),
(101, 123456788, 6000, 4000, 2000),
(102, 123456777, 7000, 3000, 4000),
(103, 123456666, 8000, 2000, 6000),
(103, 123456666, 9000, 1000, 8000),
(105, 123444444, 1000, 1000, 0);

insert into employeeYear
(E_ID, e_year, salary, salarytype, performance)
values
(100, 2021, 35000, 'W2', 'ok'),
(101, 2021, 5000, 'hourly', 'ok'),
(102, 2021, 25000, 'W2', 'ok'),
(103, 2021, 25000, 'hourly', 'ok'),
(103, 2021, 25000, 'W2', 'ok'),
(105, 2021, 25000, 'hourly', 'ok');

insert into benefits
(E_ID, SSN, benefittype, e_year, employeecontribution, employercontribution)
values
(100, 123456789, '401k', 2021, 5000, 0),
(101, 123456788, null, 2021, null, null),
(102, 123456777, null, 2021, null, null),
(103, 123456666, '401k', 2021, 0, 5000),
(103, 123456666, '401k', 2021, 5000, 5000),
(105, 123444444, '401k', 2021, 5000, 5000);

insert into stateTax
(statename, e_year, statetaxrate)
values
('Ohio', 2021, 0.0575),
('Illinois', 2021, 0.0625),
('Florida', 2021,  0.06),
('Kansas', 2021, 0.065);


insert into brackets
(bracketname, e_year, bracketrate)
values
('opt1', 2021, 0.1),
('opt2', 2021, 0.05),
('opt3', 2021, 0.075);

insert into bonus
(E_ID, e_year, percentage, performance, companySale)
values
(100, 2021, 1.5, 'W2', 'manager', 5000),
(102, 2021, 0.5, 'W2', 'ok', 5000),
(103, 2021, 0.5, 'W2', 'ok', 5000);

insert into insurancePlan
(e_id,e_year,insurancetype, premium, employerContribution)
values
(101, 2021, 'EPO', 5000, 5000),
(104, 2021, 'HMO', 2500, 2500),
(105, 2021, 'HMO', 4000, 2000);

insert into dependent
(d_id, e_id, d_name, ssn, relation, benefits)
values
(200, 100, 'Jr Turner', 123123123, 'father', 'healthcare'),
(201, 101, 'X AE A Xii', 321321321, 'son', 'healthcare');
