use crate::db::schema::students;
use crate::error_handler::CustomError;
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, select};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{connection::get_db_connection, schema::students::dsl::*};

use super::models::Student;

impl Student {
    pub async fn get_all_students() -> Result<Vec<Student>, CustomError> {
        let conn = &mut get_db_connection()?;
        let students_res = students.load::<Student>(conn)?;

        Ok(students_res)
    }

    pub async fn add_student(new_student: &mut Student) -> Result<(), CustomError> {
        let conn = &mut get_db_connection()?;
        new_student.id = Uuid::new_v4();

        let result = insert_into(students).values(&*new_student).execute(conn)?;

        Ok(())
    }
}
