use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
}
