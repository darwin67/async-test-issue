use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, Result};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use futures::future::{ok, Ready};
use lazy_static::lazy_static;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool")
    };
}

embed_migrations!();

pub struct DbConn {
    pub conn: DBConnection,
}

impl DbConn {
    pub fn with(conn: DBConnection) -> Self {
        Self { conn }
    }
}

pub struct DbPool;

impl DbPool {
    /// Use within request handlers
    pub fn req_conn() -> DbConn {
        POOL.get()
            .map(DbConn::with)
            .expect("Failed to get connection from POOL")
    }

    /// Use when need the connection object directly
    pub fn conn() -> DBConnection {
        POOL.get().expect("Failed to get connection from POOL")
    }
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = DbPool::conn();
    println!("Initializing database");

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run migrations");
}

impl FromRequest for DbConn {
    type Error = Error;
    type Future = Ready<Result<DbConn, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ok(DbPool::req_conn())
    }
}
