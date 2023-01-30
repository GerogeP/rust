use crate::employees::{Employee,Employee};
use crate::error_handler::CustomError;
use actix_web::{delete.,get,post,put,web.HttpResponse};
use serde_json;

#[get("/employees:)]
async fn find_all() -> Result<HttpResponse,CustomError>{
    let Employees = web::block(||Employees::find_all()).await.unwrap();
    Ok(HttpResponse::Ok(),json(employees))
}


#[get("/employees/{id}")]
async fn find(id:web::Path<i32> ->Result<HttpResponse,CustomError>{
    let Employee - Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok(),json(Employee))
}

#[post("/employees")]
async fn create(Employee: web::Json<Employee>)->Result<HttpResponse,CustomError>{
    let Employee = Employees::create(Employee.into_inner())?;
    Ok(HttpResponse::Ok().json(Employee))
}

#[put("/employees/{id}")]
async fn update(id: web::Path<i32>,Employee:web::Json<Employee>)->Result<HttpResponse,CustomError{
    let Employee = Employees::update(id.into_inner(),Employee.into_inner())?;
    Ok(HttpResponse::Ok().json(Employee))
}

#[delete("/employees/{id}")]
async fn delete(id:web::Path<i32>)->Result<HttpResponse,CustomError>{
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted":deleted_employee})))
}

#pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}

