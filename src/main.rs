extern crate asynctest;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    asynctest::config::init();

    HttpServer::new(move || App::new().configure(asynctest::config::routes::routes))
        .bind("[::]:4000")?
        .run()
        .await
}
