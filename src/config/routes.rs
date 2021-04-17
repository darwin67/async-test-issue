use crate::controllers as ctr;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/users")
                .route("", web::get().to(ctr::users::Endpoint::list))
                .route("", web::post().to(ctr::users::Endpoint::create))
                .route("{id}", web::get().to(ctr::users::Endpoint::show))
                .route("{id}", web::delete().to(ctr::users::Endpoint::delete)),
        ),
    );
}
