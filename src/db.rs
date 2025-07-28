use rocket_db_pools::{mongodb::Client, Database};

#[derive(Database)]
#[database("local")]
pub struct MainDatabase(Client);