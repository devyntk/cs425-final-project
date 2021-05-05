create table employee(
	E_ID INT,
	SSN VARCHAR(9) not null unique,
	firstName VARCHAR (20) not null,
	lastName VARCHAR (20) not null,
	jobTitle VARCHAR (20),
    stateAddress VARCHAR(2),
	primary key(E_ID)
);
create type perfomance AS ENUM ('well', 'ok', 'not_well', 'super_performer', 'manager');
create type salary AS ENUM ('w2', 'hourly');
create table employeeYear(
	E_ID INT,
	e_year INT not NULL,
	salary REAL,
	salaryType salary,
	performance perfomance,
	foreign key (E_ID) references employee(E_ID),
    PRIMARY KEY (E_ID, e_year)
);

create table socialSecurity(
    E_ID INT,
    e_year INT not NULL,
    amount REAL not null,
    employeePays REAL,
    employerPays REAL,
    foreign key (E_ID, e_year) references employeeYear(E_ID, e_year),
    PRIMARY KEY (E_ID, e_year)
);
create table benefits(
    E_ID INT,
    e_year INT not NULL,
	benefitType VARCHAR (20) not NULL,
	employeeContribution REAL not NULL,
	employerContribution REAL,
    foreign key (E_ID, e_year) references employeeYear(E_ID, e_year),
    PRIMARY KEY (E_ID, e_year)
);
create table stateTax(
	stateName VARCHAR(2) not null UNIQUE,
	e_year INT not NULL,
	stateTaxRate REAL
);
create table brackets(
	bracketName VARCHAR (20) not null UNIQUE,
	e_year INT not NULL,
	bracketRate REAL
);
create table bonus(
	E_ID INT,
	e_year INT not NULL,
	percentage REAL,
	company_sale REAL,
    foreign key (E_ID, e_year) references employeeYear(E_ID, e_year),
    PRIMARY KEY (E_ID, e_year)
);
create table insurancePlan(
	E_ID INT,
	e_year INT not NULL,
	insuranceType VARCHAR (20),
	premium REAL,
	employerContribution REAL,
    foreign key (E_ID, e_year) references employeeYear(E_ID, e_year),
    PRIMARY KEY (E_ID, e_year)
);
create table dependent(
	D_ID INT,
	E_ID INT,
	d_name VARCHAR (20),
	SSN VARCHAR(9) not null unique,
	relation VARCHAR (20),
	benefits VARCHAR(20),
	foreign key (E_ID) references employee(E_ID),
	PRIMARY KEY (E_ID, D_ID)
);

create index ssn_index on employee(SSN);
create index salary_index on employeeYear(salaryType);
create index stateTax_index on stateTax(stateTaxRate);
create index bracket_index on brackets(bracketRate);
create index socialSecurity_index on socialSecurity(employeePays);
create index benefit_index on benefits(employeeContribution);
create index insurance_index on insurancePlan(premium);
create index bonus_index on bonus(percentage);
create index employer_socialSecurity_index on socialSecurity(employerPays);
create index employer_benefit_index on benefits(employerContribution);

/* here goes all the functions to be used for admin reports*/

/* "getters" */
/*find employee ssn by employee_ID*/
create function find_employee_ssn(employee_ID int)
returns numeric (9,0) as $$
	declare SSN_val VARCHAR(9) ;
    begin
		select SSN into SSN_val from employee where E_ID=employee_ID;
	return SSN_val;
end;
$$ language plpgsql;
/*find salary by employee id and year*/
create function find_salary(employee_ID int, yr DATE)
returns numeric (10,2) as $$
    declare salary_val numeric (10,2);
    begin
		select salary into salary_val from employeeYear where E_ID=employee_ID and e_year=yr;
	return salary_val;
end;
$$ language plpgsql;

/*find tax rate using employee id and year to find state tax rate */
create function findTaxRate(employee_ID int, yr DATE)
returns decimal as $$
    declare rate decimal;
    declare name_of_state varchar(20);
    begin
        select stateAddress into name_of_state from employee where E_ID=employee_ID;
		select stateTaxRate into rate from stateTax where e_year=yr and stateName=name_of_state;
	return rate;
end;
$$ language plpgsql;

/*find bracket using year*/
create function findBracket(yr DATE)
returns decimal as $$
    declare b_rate decimal;
    begin
		select bracketRate into b_rate from brackets where e_year=yr;
	return b_rate;
end;
$$ language plpgsql;

/* set tax value */
create function stateTax(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare tax_val numeric (10,2);
    begin
        tax_val := findTaxrate(employee_ID, yr) * find_salary(employee_ID, yr);
    end;
$$ language plpgsql;
/* set bracket value */
create function bracket(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare bracket_val numeric (10,2);
    begin
	    bracket_val := findBracket(yr) * find_salary(employee_ID, yr);
    end;
$$ language plpgsql;
/* set social security contribution */
create function socialSec(employee_ID int)
returns numeric(10,2) as $$
    declare ssn_val numeric (10,2);
    begin
        select amount into ssn_val from socialSecurity where employee_ID = E_ID;
        return ssn_val;
    end;
$$ language plpgsql;

/* calculate tax reductions for weekly paycheck report*/
create function tax_reductions(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare tax_red_val numeric(10,2);
    begin
        tax_red_val := stateTax(employee_ID, yr) + bracket(employee_ID, yr) + socialSec(employee_ID) ;
        return tax_red_val;
    end;
$$ language plpgsql;

/*employee_contribution to 401k*/
create function Val401k(employee_ID int)
returns numeric(10,2) as $$
    declare contribution numeric(10,2);
    declare benefit_type varchar(20);
    begin
        select benefitType into benefit_type from benefits where E_ID=employee_ID;
        select employeeContribution into contribution from benefits where benefit_type='401k' and employee_ID=E_ID;
        return contribution;
    end
$$ language plpgsql;

/*employee_contribution to medicare*/
create function medicare(employee_ID int)
    returns numeric(10,2) as $$
    declare contribution numeric(10,2);
    declare benefit_type varchar(20);
    begin
        select benefitType into benefit_type from benefits where E_ID=employee_ID;
        select employeeContribution into contribution from benefits where benefit_type='medicare' and employee_ID=E_ID;
        return contribution;
    end
$$ language plpgsql;

/*find insurance premium cost using employee_ID and specific year*/
create function insurance_premium(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare premiumcost numeric(10,2);
begin
	select premium into premiumcost from insurancePlan where employee_ID=E_ID and e_year=yr;
	return premiumcost;
end;
$$ language plpgsql;

/*to generate paycheck*/
create function paycheck(employee_ID int, yr DATE)
    returns numeric(10,2) as $$
    declare paycheck_amount numeric(10,2);
begin
	paycheck_amount := find_salary(employee_ID, yr) - tax_reductions(employee_ID, yr) - Val401k(employee_ID) - insurance_premium(employee_ID, yr);
	return paycheck_amount;
end;
$$ language plpgsql;

/*create w2 report requires: annual income, deductions and bonus */
/*calculate yearly income of employee*/
create function yearly_income(employee_ID int, yr Date)
returns numeric(10,2) as $$
    declare annual_income numeric(10,2);
    begin
		annual_income := (find_salary(employee_ID, yr))*24;
		/* salary is biweekly, so income for a year includes 12 * 2 salaries */
        return annual_income;
    end;
$$ language plpgsql;

/*create table of all the deductions */
create function deductions(employee_ID int, yr DATE)
    returns numeric(10,2) as $$
begin 
	return tax_reductions(employee_ID, yr) + Val401k(employee_ID) + insurance_premium(employee_ID, yr);
end;
$$ language plpgsql;

/*calculates bonus rate depending on performance*/
create function percentage_bonus(perf_type varchar(20))
returns decimal as $$
    begin
	if perf_type == 'well' then
		return 1.0;
	else if perf_type == 'okay' then
		return 0.5;
	else if perf_type == 'super_performer' or perf_type == 'manager' then 
		return 1.5;
	else
		return 0.5;
	end if;
	end if;
	end if;
	end;
$$ language plpgsql;
/*calculates bonus depending on employee id, bonus rate, and year*/
create function bonus_earned(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare s_type varchar(20);
    declare bonus numeric(10,2);
    declare perf varchar(20);
begin
    select performance into perf from employeeYear where E_ID=employee_ID and yr=e_year;
    select salaryType into s_type from employeeYear where E_ID=employee_ID and e_year=yr;
        if s_type == 'W2' then
            bonus = yearly_income(employee_ID, yr) * percentage_bonus(perf);
        else
            bonus = 0;
        end if;
    return bonus;
end;
$$ language plpgsql;

/*w2 report data*/
create function w2_report(employee_ID int, yr DATE)
    returns numeric(10,2) as $$
    begin
        return yearly_income(employee_ID, yr) + deductions(employee_ID, yr)- bonus_earned(employee_ID, yr);
    end
$$ language plpgsql;

/*company employee expense report:wages, bonus paid, 401k employer contribution, ssn contribution, insurance contribution*/
create function find_wages(yr DATE)
    returns SETOF integer AS $BODY$
    begin
        return query select E_ID, salary from employeeYear where e_year=yr;
    end
$BODY$ language plpgsql;

create function bonus_paid(yr DATE)
    returns SETOF integer AS $BODY$
    begin
        return query select E_ID, salary, performance from employeeYear where e_year=yr;
    end
$BODY$ language plpgsql;

create function retirement_employer(yr DATE)
returns SETOF integer AS $BODY$
    begin
        return query select E_ID, employerContribution from benefits where e_year=yr and benefitType='401k';
    end
$BODY$ language plpgsql;

create function ssn_employer(yr DATE)
    returns SETOF integer AS $BODY$
    begin
        return query select E_ID, employerPays from socialSecurity where e_year=yr;
    end
$BODY$ language plpgsql;

create function insurance_employer(yr DATE)
    returns SETOF integer AS $BODY$
    begin
        return query select E_ID, employerContribution from insurancePlan where e_year=yr;
    end
$BODY$ language plpgsql;



