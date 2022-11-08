use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::db::schema::{app_users, students};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
pub struct AppUser {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub create_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
pub struct Student {
    pub id: uuid::Uuid,
    pub lastname: String,
}
