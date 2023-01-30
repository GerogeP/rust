use crate::db;
use crate::error_handler::CustomError;
use crate::schema::employees;
use disel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,AsChangeset,Insertable)]
#[table_name="employees"]
pub struct Employee{
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary:i32,
    pub age:i32,
}


#[derive(Serialize,Deserialize,AsChangeset,Insertable)]
pub struct Employees{
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary:i32,
    pub age:i32,
}

impl Employees{
    pub fn find_all() -> Result<Vec<self>,CustomError>{
        let conn = db::connection()?;
        let employees = employees::table.load::<Employees>($conn)?;
        OK(employees);
    }

    pub fn find(id:i32)->Result<Self,CustomError>{
        let conn=db::connection()?;
        let employee = employees::table.filter(employees::id.eq(id)).first(&conn)?;
        OK(employee)
    }

    pub fn create(employee:Employee) -> Result<Self,CustomError>{
        let conn = db:connection()?;
        let employee:Employee = Employee::from(t:employee);
        let employee = disel::insert_into(employee::table).value(employee).get_result)&conn)?;
        OK(employee)
    }

    pub fn update(id:i32,employee:Employee)->Result<Self,CustomError>{
        let conn = db::connection()?;
        let employee = disel::update(employee::table).filter(employees::id.eq(id)).set(employee).get_result(&conn)?:;
        OK(employee)
    }

    pub fn delete(id:i32) -> Result<usize,CustomError>{
        let conn = db::connection()?;
        let res = disel::delete(employee::table.filter(employees::id.eq(id))).excute(&&conn)?;
        OK(res)
    }


}

impl Employee{
    fn from(employee:Employee) -> Employee{
        Employee{
            first: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary.
                age: employee.age,
        }
    }
}

