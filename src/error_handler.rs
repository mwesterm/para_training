use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use argon2::PasswordHash;
use diesel::result::Error as DieselError;
use log::*;
use serde_json::json;

use thiserror;

use crate::db::schema::app_users::password;

#[derive(Debug, thiserror::Error)]
pub enum ORMError {
    #[error("ORM Record not found")]
    ORMNotFound,
    #[error("Duplicate Record")]
    ORMNotUnique,
    #[error("ORM Database-Error")]
    ORMDatabaseError(String),
    #[error("ORM Unknowen error")]
    ORMUnexpectedError,

    #[error("Invalid credentials")]
    AuthInvalidCredentials,
    #[error("Auth Unexpected Error")]
    AuthUnexpectedError,
    #[error("Not authorized")]
    AuthNotAuthorized,
}

impl From<DieselError> for ORMError {
    fn from(error: DieselError) -> Self {
        let result: ORMError = match error {
            DieselError::DatabaseError(_, err) => {
                ORMError::ORMDatabaseError(err.message().to_string())
            }
            //DieselError::NotFound => ORMError::NotFound,
            err => ORMError::ORMUnexpectedError,
        };
        error!("Diesel-Error: {}", result);
        result
    }
}

impl ResponseError for ORMError {
    fn error_response(&self) -> HttpResponse {
        let status_code: StatusCode;
        let error_message: &str;
        match self {
            ORMError::ORMNotFound => {
                status_code = StatusCode::NOT_FOUND;
                error_message = "Record not found";
            }
            ORMError::ORMNotUnique => {
                status_code = StatusCode::NOT_ACCEPTABLE;
                error_message = "Record not unique";
            }
            ORMError::ORMDatabaseError(_) => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                error_message = "Database Error";
            }
            ORMError::ORMUnexpectedError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                error_message = "ORM Unexpected Error";
            }
            ORMError::AuthInvalidCredentials => {
                status_code = StatusCode::UNAUTHORIZED;
                error_message = "Invalid Credentials";
            }
            ORMError::AuthUnexpectedError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                error_message = "Auth Unexpected Error";
            }
            ORMError::AuthNotAuthorized => {
                status_code = StatusCode::UNAUTHORIZED;
                error_message = "No Access rights";
            }
        }
        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
