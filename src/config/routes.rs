use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(web::scope("/users").route("", web::get().to(HttpResponse::Accepted()))),
        ),
    );
}
