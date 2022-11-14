use crate::db;
use crate::db::schema::students;

use crate::error_handler::ORMError;
use diesel::prelude::*;

use uuid::Uuid;

use super::models::Student;
use crate::db::{connection::get_db_connection, schema::students::dsl::*};

impl Student {
    pub async fn get_all_students() -> Result<Vec<Student>, ORMError> {
        let conn = &mut get_db_connection()?;
        let students_res = students.load::<Student>(conn)?;
        Ok(students_res)
    }

    pub async fn add_student(new_student: Student) -> Result<Self, ORMError> {
        let conn = db::connection::get_db_connection()?;

        new_student.id = Uuid::new_v4();
        let new_student = Student::from(new_student);

        let new_stundent = diesel::insert_into(students::table)
            .values(new_student)
            .get_result(&conn)?;

        Ok(new_student)
    }

    pub async fn get_students_by_id(search_id: uuid::Uuid) -> Result<Student, ORMError> {
        let conn = &mut get_db_connection()?;
        let student = students::table
            .filter(students::id.eq(search_id))
            .first(conn)?;
        Ok(student)
    }
}
