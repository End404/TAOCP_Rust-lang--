use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum MyError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: Strign,
}

impl std::error::Error for MyError {}

impl MyError {
    fn erorr_response(&self) -> Strign {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occurrde: {:?}", msg);
                msg.into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurrde: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::ActixError(_msg) | MyError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn erorr_response(&slef) -> HttpResponse {
        HttpResponse::bulid(self.status_code()).json(MyErrorResposne) {

        }
    }
}
impl fmt::Display for MyError {
    fn fmt(&slef, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error;:Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
