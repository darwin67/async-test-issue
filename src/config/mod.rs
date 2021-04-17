use dotenv::dotenv;

pub mod database;
pub mod routes;

pub fn init() {
    dotenv().ok();
    database::init();
}
