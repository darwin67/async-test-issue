use crate::config::database::DbConn;
use crate::models::user::{NewUser, User};
use crate::utils::errors::ServiceError;
use actix_web::{error::BlockingError, web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: i32,
}

pub struct Endpoint;

impl Endpoint {
    pub async fn list(db: DbConn) -> Result<HttpResponse, ServiceError> {
        web::block(move || {
            let conn = db.conn;
            Ok(User::get_all(&conn))
        })
        .await
        .map(|users| HttpResponse::Ok().json(json!({ "data": users })))
        .map_err(|err| match err {
            BlockingError::Canceled => ServiceError::InternalError,
            BlockingError::Error(service_error) => service_error,
        })
    }

    pub async fn create(
        db: DbConn,
        data: web::Json<NewUser>,
    ) -> Result<HttpResponse, ServiceError> {
        web::block(move || {
            let conn = db.conn;

            data.save(&conn).map_err(|_| ServiceError::BadRequest)
        })
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| match err {
            BlockingError::Canceled => ServiceError::InternalError,
            BlockingError::Error(service_error) => service_error,
        })
    }

    pub async fn show(db: DbConn, path: web::Path<UserInfo>) -> Result<HttpResponse, ServiceError> {
        web::block(move || {
            let conn = db.conn;

            match User::get_by_id(&conn, path.id) {
                None => Err(ServiceError::NotFound),
                Some(user) => Ok(user),
            }
        })
        .await
        .map(|user| HttpResponse::Ok().json(json!({ "data": user })))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalError,
        })
    }

    pub async fn delete(
        db: DbConn,
        path: web::Path<UserInfo>,
    ) -> Result<HttpResponse, ServiceError> {
        web::block(move || {
            let conn = db.conn;
            User::delete(&conn, path.id).map_err(|_| ServiceError::InternalError)
        })
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|err| match err {
            BlockingError::Canceled => ServiceError::InternalError,
            BlockingError::Error(service_error) => service_error,
        })
    }
}
