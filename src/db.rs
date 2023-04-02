use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

#[derive(Database)]
#[database("sps_mysql")]
pub struct SPS(sqlx::MySqlPool);
