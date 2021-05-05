
create table user_tbl(
     username varchar(32) not null unique,
     psswrd varchar (64) not null,
     IsAdmin boolean not null,
     IsEmployee boolean not null,
     IsEmployer boolean not null,
     HasDependent boolean not null,
     E_ID INT not null,
     foreign key (E_ID) references employee(E_ID),
     primary key(E_ID)
);
insert into employee
(E_ID, SSN, firstName, lastName, jobTitle, stateAddress)
values
(100, 123456789, 'Paige', 'Turner', 'Manager', 'IL' ),
(101, 123456788, 'Elon', 'Musk', 'Salesperson', 'OH'),
(102, 123456777, 'May', 'Day', 'Salesperson', 'FL'),
(103, 123456666, 'Al', 'Dente', 'Salesperson', 'IL'),
(104, 123455555, 'Devyn', 'Keeney', 'Robotics_consultant', 'KS'),
(105, 123444444, 'Khushboo', 'Bhattu', 'Agent_of_Chaos', 'OH');

insert into user_tbl
(username, psswrd, IsAdmin, IsEmployee, IsEmployer, HasDependent, E_ID)
values
('devyn', 'password', false, true, false, false, 104),
('khushboo', 'password', true, false, false, false, 105),
('musky', 'password', false, false, true, true, 101);

insert into employeeYear
(E_ID, e_year, salary, salarytype, performance)
values
(100, 2021, 35000, 'w2', 'ok'),
(101, 2021, 5000, 'hourly', 'ok'),
(102, 2021, 25000, 'w2', 'ok'),
(103, 2020, 25000, 'hourly', 'ok'),
(103, 2021, 25000, 'w2', 'ok'),
(104, 2021, 30000, 'w2', 'ok'),
(104, 2020, 25000, 'hourly', 'not_well'),
(105, 2021, 25000, 'hourly', 'ok');

insert into socialSecurity
(E_ID, e_year, amount, employeepays, employerPays)
values
(100, 2021, 5000, 5000, 0),
(101, 2021, 6000, 4000, 2000),
(102, 2021, 7000, 3000, 4000),
(103, 2021, 8000, 2000, 6000),
(103, 2020, 9000, 1000, 8000),
(105, 2021, 1000, 1000, 0),
(104, 2021, 1000, 500, 500),
(104, 2020, 1000, 750, 250);


insert into benefits
(E_ID, e_year, benefittype, employeecontribution, employercontribution)
values
(100, 2021, '401k', 5000, 0),
(103, 2021, '401k', 0, 5000),
(103, 2020, '401k', 5000, 5000),
(104, 2021, '401k', 250, 250),
(104, 2020, '401k', 400, 400),
(105, 2021, '401k', 5000, 5000);

insert into stateTax
(statename, e_year, statetaxrate)
values
('OH', 2021, 0.0575),
('IL', 2021, 0.0625),
('FL', 2021,  0.06),
('KS', 2021, 0.065);


insert into brackets
(bracketname, e_year, bracketrate)
values
('opt1', 2021, 0.1),
('opt2', 2021, 0.05),
('opt3', 2021, 0.075);

insert into bonus
(E_ID, e_year, percentage, company_sale)
values
(100, 2021, 1.5, 5000),
(102, 2021, 0.5, 5000),
(104, 2021, 1.0, 1000),
(103, 2021, 0.5, 5000);

insert into insurancePlan
(e_id,e_year,insurancetype, premium, employerContribution)
values
(101, 2021, 'EPO', 5000, 5000),
(103, 2021, 'HMO', 2500, 2500),
(104, 2021, 'EPO', 3000, 2000),
(105, 2021, 'HMO', 4000, 2000);

insert into dependent
(d_id, e_id, d_name, ssn, relation, benefits)
values
(200, 100, 'Jr Turner', 123123123, 'father', 'healthcare'),
(201, 101, 'X AE A Xii', 321321321, 'son', 'healthcare');






