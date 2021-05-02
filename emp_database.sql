create table employee(
	E_ID INT,
	SSN NUMERIC (9,0) not null unique,
	firstName VARCHAR (20) not null,
	lastName VARCHAR (20) not null,
	jobTitle VARCHAR (20),
	primary key(E_ID),
	foreign key (SSN) references socialSecurity(SSN)
);
create table socialSecurity(
	SSN NUMERIC (9,0) not null unique,
	E_ID INT,
	amount NUMERIC (10,2) not null,
	employeePays numeric (10,2),
	employerPays numeric (10,2),
	primary key(SSN),
	foreign key (E_ID) references employee(E_ID)
);
create table employeeYear(
	E_ID INT,
	e_year DATE not NULL,
	salary NUMERIC (10,2),
	salaryType VARCHAR (20),
	performance VARCHAR (20), /* 5 options : well, ok, not_well, super_performer, manager*/
	constraint emp_year primary key (E_ID, e_year),

	foreign key (E_ID) references employee(E_ID)
);
create table benefits(
	SSN NUMERIC (9,0) not null unique,
	benefitType VARCHAR (20) not NULL,
	e_year DATE not NULL,
	employeeContribution NUMERIC (10,2) not NULL,
	employerContribution NUMERIC (10,2),
	foreign key (SSN) references socialSecurity(SSN),
	foreign key (E_ID) references employee(E_ID)
);
create table stateTax(
	stateName VARCHAR(20) not null UNIQUE,
	e_year DATE not NULL,
	stateTaxRate DECIMAL
);
create table brackets(
	bracketName VARCHAR (20) not null UNIQUE,
	e_year DATE not NULL,
	bracketRate DECIMAL
);
create table bonus(
	E_ID INT,
	e_year DATE not NULL,
	percentage DECIMAL,
	performance VARCHAR (20),
	company_sale NUMERIC (10,2),
	foreign key (E_ID) references employee(E_ID)
);
create table insurancePlan(
	E_ID INT,
	e_year DATE not NULL,
	insuranceType VARCHAR (20),
	premium NUMERIC (10,2),
	employerContribution numeric (10,2),
	foreign key (E_ID) references employee(E_ID)
);
create table dependent(
	D_ID INT,
	E_ID INT,
	d_name VARCHAR (20),
	SSN NUMERIC (9,0) not null unique,
	relation VARCHAR (20),
	benefits VARCHAR(20),
	foreign key (E_ID) references employee(E_ID)
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
	declare SSN_val numeric(9,0);
    begin
		select SSN into SSN_val from employee where E_ID=employee_ID;
	return SSN_val;
end;
$$;
/*find salary by employee id and year*/
create function find_salary(employee_ID int, yr DATE)
returns numeric (10,2) as $$
    declare salary_val numeric (10,2);
    begin
		select salary into salary_val from employeeYear where E_ID=employee_ID and e_year=yr;
	return salary_val;
end;
$$;

/*find tax rate using employee id and year to find state tax rate */
create function findTaxRate(employee_ID int, yr DATE)
returns decimal as $$
    declare rate decimal;
    begin
		select stateTaxRate into rate from employeeYear where E_ID=employee_ID and e_year=yr;
	return rate;
end;
$$;

/*find bracket using year*/
create function findBracket(yr DATE)
returns decimal as $$
    declare b_rate decimal;
    begin
		select bracketRate into b_rate from brackets where e_year=yr;
	return b_rate;
end;
$$;

/* set tax value */
create function stateTax(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare tax_val numeric (10,2);
    begin
        tax_val := findTaxrate(employee_I, yr) * findsalary(employee_I, yr);
    end;
$$;
/* set bracket value */
create function bracket(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare bracket_val numeric (10,2);
    begin
	    bracket_val := findBracket(yr) * findsalary(employee_I, yr);
    end;
$$;
/* set social security contribution */
create function socialSec(employee_ID int)
returns numeric(10,2) as $$
    declare ssn_val numeric (10,2);
    begin
        select amount into ssn_val from socialSecurity where employee_ID = E_ID;
        return ssn_val;
    end;
$$;

/* calculate tax reductions for weekly paycheck report*/
create function tax_reductions(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare tax_red_val numeric(10,2);
    begin
        tax_red_val := stateTax(employeeID, yr) + bracket(employeeID, yr) + socialSec(employeeID) ;
        return tax_red_val;
    end;
$$;

/*employee_contribution to 401k*/
create function Val401k(employee_ID int, benefitType varchar)
returns numeric(10,2) as $$
    declare contribution numeric(10,2);
    begin
        select employeeContribution into contribution from benefits where benefitType="401k" and employee_ID=E_ID;
        return contribution;
    end;
$$;

/*find insurance premium cost using employee_ID and specific year*/
create function insurance_premium(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare premiumcost numeric(10,2);
begin
	select premium into premiumcost from insurancePlan where employee_ID=E_ID and e_year=yr;
	return premiumcost;
end;
$$;

/*to generate paycheck*/
create function paycheck(employeeID int, yr DATE)
returns table as $$
    declare paycheck_amount numeric(10,2)
begin
	paycheck_amount := find_salary(employeeID, yr) - tax_reductions(employeeID, yr) - 401k(employeeID, yr) - insurance_premium(employeeID, yr);
	return employeeID, paycheck;
end;
$$;

/*create w2 report requires: annual income, deductions and bonus */
/*calculate yearly income of employee*/
create function yearly_income(employee_ID int, yr Date)
returns numeric(10,2) as $$
    declare annual_income numeric(10,2);
    begin
		annual_salary := (find_salary(employee_ID, yr))*24;
		/* salary is biweekly, so income for a year includes 12 * 2 salaries */
        return annual_salary
    end;
$$;

/*create table of all the deductions */
create function deductions(employee_ID int, yr DATE)
returns table as $$
begin 
	return tax_reductions(employee_ID, yr) , 401k(employee_ID, yr) , insurance_premium(employee_ID, yr);
end;
$$;

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
$$;
/*calculates bonus depending on employee id, bonus rate, and year*/
create function bonus_earned(employee_ID int, yr DATE)
returns numeric(10,2) as $$
    declare s_type varchar(20);
    declare bonus numeric(10,2);
    declare perf varchar(20);
    begin
        select performance from employeeYear where E_ID=employee_ID and yy=e_year;
        select salaryType into s_type from employeeYear where E_ID=employee_ID and e_year=yr;
            if s_type == 'W2' then
                bonus = yearly_income(emloyee_ID, yr) * percentage_bonus(performance);
            else:
                bonus := 0;
            end if;
        return bonus;
    end;
$$;

/*w2 report data*/
create function w2_report(employee_ID int, yr DATE)
returns table as $$
    begin
        return find_employee_SSN(employee_ID), yearly_income(employee_ID, yr), deductions(employee_ID, yr), bonus_earned(employee_ID, yr)
    end
$$;

/*company employee expense report:wages, bonus paid, 401k employer contribution, ssn contribution, insurance contribution*/
create function find_wages(yr DATE)
returns table as $$
    begin
        return query select employee_ID, salary from employeeYear where e_year=yr;
    end
$$;

create function bonus_paid(yr DATE)
returns table as $$
    begin
        return query select employee_ID, salary, performance from employeeYear where e_year=yr;
    end
$$;

create function retirement_employer(yr DATE)
returns table as $$
    begin
        return query select employee_ID, employerContribution from benefits where e_year=yr and benefitType='401k';
    end
$$;

create function ssn_employer(yr DATE)
returns table as $$
    begin
        return query select employee_ID, employerPays from socialSecurity where e_year=yr;
    end
$$;

create function insurance_employer(yr DATE)
returns table as $$
    begin
        return query select employee_ID, employerContribution from insurancePlan where e_year=yr;
    end
$$;


