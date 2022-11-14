use crate::db::schema::app_users;

use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::db::{connection::get_db_connection, schema::app_users::dsl::*};
use crate::error_handler::ORMError;

use super::connection::get_db_Mut_connection;
use super::models::AppUser;
/*
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
*/

impl AppUser {
    pub async fn get_all_app_users() -> Result<Vec<AppUser>, ORMError> {
        let conn = &mut get_db_connection()?;
        let app_users_res = app_users.load::<AppUser>(conn)?;
        Ok(app_users_res)
    }

    pub async fn find_by_name(search_user_name: &str) -> Result<AppUser, ORMError> {
        let conn = &mut get_db_connection()?;
        let app_users_res = app_users::table
            .filter(username.eq(search_user_name))
            .first::<AppUser>(conn)?;
        Ok(app_users_res)
    }

    pub async fn add_app_user(new_app_user: AppUser) -> Result<(), ORMError> {
        //        let conn = get_db_connection()?;.
        let conn = &mut get_db_Mut_connection().get().unwrap();
        //Don't add username twice
        match AppUser::find_by_name(&new_app_user.username).await {
            Ok(_) => return Err(ORMError::ORMNotUnique),
            _ => (),
        }

        new_app_user.id = Uuid::new_v4();

        let new_app_user = AppUser::from(new_app_user);

        let new_app_user = diesel::insert_into(app_users::table)
            .values(new_app_user)
            .get_result(conn)?;

        Ok(())
    }
}
