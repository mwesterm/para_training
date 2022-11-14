use crate::error_handler::ORMError;

use diesel::{r2d2::ConnectionManager, PgConnection};
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init_db_connectons() {
    lazy_static::initialize(&POOL);
    get_db_connection().expect("Failed to get db connection"); //Test to get connection
}

pub fn get_db_connection() -> Result<DbConnection, ORMError> {
    POOL.get()
        .map_err(|e| ORMError::ORMDatabaseError(format!("Failed getting db connection: {}", e)))
}

pub fn get_db_Mut_connection() -> Pool {
    POOL.clone()
}
