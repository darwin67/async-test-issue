use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "BadRequest")]
    BadRequest,
    #[display(fmt = "Internal Server Error")]
    InternalError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest => HttpResponse::BadRequest().finish(),
            ServiceError::NotFound => HttpResponse::NotFound().finish(),
            &ServiceError::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}
