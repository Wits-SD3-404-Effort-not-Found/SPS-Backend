use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

// Rocket DB integration setup 
#[derive(Database)]
#[database("sps_mysql")]
pub struct SPS(sqlx::MySqlPool);

// Exists in one file just to keep things somewhat organized. Realistically might add functions 
// in here eventually
