use std::fmt;
use std::fmt::Formatter;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseError};
use diesel::result::Error as DieselError;
use serde_json::json;

#[derive(Debug,Deserialize)]
pub struct CustomError{
    pub error_status_code:u16,
    pub erroe_message:String
}

impl CustomError{
    pub fn new(error_status_code:u16,erroe_message:String)->CustomError{
        CustomError{
            error_status_code,
            erroe_message,
        }
    }
}

impl fmt::Display for CustomError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(data:self.erroe_message.as_str())
    }
}

impl From<DieselError> for CustomError{
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_,error)=>CustomError::new(error_status_code:409,erroe_message:err.messagr().to_string()),
            DieselError::NotFound => {
                CustomError(404, "employee not found".to_string())
            }
            err=>CustomError::new(error_status_code: 500, erroe_message: format!("unknown error:{}",err))
        }
    }
}


impl HttpResponseError for CustomError{
    fn error_response(&self) -> HttpResponse {
        let status_code : StatusCode =  match StatusCode::from_u16(src: self.error_status_code){
            OK(status_code : StatusCode ) => status_code,
            Err(_)->StatusCode::INTERNAL_SERVER_ERROR,
        };

        let erroe_message : String = match status_code.as_u16()<500{
            true=>self.erroe_message.clone(),
            false=>"internal server error".to_string(),
        };

        HttpResponse::build(status: status_code).json(value:json!({"message":erroe_message});
                                                      }
                                                      }



