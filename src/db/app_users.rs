use crate::db::schema::app_users;
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::{connection::get_db_connection, schema::app_users::dsl::*};
use crate::error_handler::CustomError;

use super::models::AppUser;

#[derive(Deserialize, Insertable)]
#[diesel(table_name=app_users)]
pub struct NewAppUser<'a> {
    #[serde(default)]
    id: uuid::Uuid,
    #[serde(default)]
    pub username: &'a str,
    #[serde(default)]
    pub password: &'a str,
    #[serde(default)]
    pub active: bool,
    pub create_date: NaiveDateTime,
}

impl AppUser {
    pub async fn get_all_app_users() -> Result<Vec<AppUser>, CustomError> {
        let conn = &mut get_db_connection()?;
        let app_users_res = app_users.load::<AppUser>(conn)?;
        Ok(app_users_res)
    }

    pub async fn add_app_user(new_app_user: &mut AppUser) -> Result<(), CustomError> {
        let conn = &mut get_db_connection()?;
        new_app_user.id = Uuid::new_v4();

        insert_into(app_users)
            .values(&*new_app_user)
            .execute(conn)?;

        Ok(())
    }
}
