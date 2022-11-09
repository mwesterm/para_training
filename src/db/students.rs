#[allow(unused_imports)]
use crate::db::schema::students;
use crate::error_handler::CustomError;
use diesel::{insert_into, prelude::*};
use log::{debug, info};
use uuid::Uuid;

use super::models::Student;
use crate::db::{connection::get_db_connection, schema::students::dsl::*};

impl Student {
    pub async fn get_all_students() -> Result<Vec<Student>, CustomError> {
        let conn = &mut get_db_connection()?;
        let students_res = students.load::<Student>(conn)?;
        Ok(students_res)
    }

    pub async fn add_student(new_student: &mut Student) -> Result<(), CustomError> {
        let conn = &mut get_db_connection()?;
        new_student.id = Uuid::new_v4();
        insert_into(students).values(&*new_student).execute(conn)?;
        debug!("Student insert OK:{:?}", new_student);
        Ok(())
    }

    pub async fn get_students_by_id(search_id: uuid::Uuid) -> Result<Student, CustomError> {
        let conn = &mut get_db_connection()?;
        let student = students::table
            .filter(students::id.eq(search_id))
            .first(conn)?;
        Ok(student)
    }
}
