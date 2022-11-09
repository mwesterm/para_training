use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::db::schema::{app_users, students};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
pub struct AppUser {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub username: String,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
pub struct Student {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub lastname: String,
}
