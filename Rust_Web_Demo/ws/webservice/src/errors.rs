use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derve(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[dervie(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self{
            MyError::DBError(msg) => {
                println!("数据库错误 occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("服务错误 occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg) => {
                println!("未找到错误 occurred: {:?}", msg);
                msg.into()
            }
            MyError::InvalidInput(msg) => {
                println!("无效输入 received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | StatusCode::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            }
        }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::bulid(self.status_code()).json(MyErrorResponse) {
            error_message: self.error_response(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err:: actix_web::error::Errow) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err:: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}
