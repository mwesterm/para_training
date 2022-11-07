use std::time::SystemTime;

use actix_web::App;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::{connection::get_db_connection, schema::app_users::dsl::*};
use crate::error_handler::CustomError;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub create_date: SystemTime,
}

impl AppUser {
    pub fn test_fn() -> bool {
        true
    }
    pub async fn get_all_app_users() -> Result<Vec<AppUser>, CustomError> {
        let conn = &mut get_db_connection()?;
        let app_users_res = app_users.load::<AppUser>(conn)?;
        Ok(app_users_res)
    }
}
