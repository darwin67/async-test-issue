extern crate asynctest;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new())
        .workers(10)
        .bind("[::]:4000")?
        .run()
        .await
}
