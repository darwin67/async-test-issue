use crate::config::database as db;
use crate::models::factory::Factory;
use crate::schema::users;
use diesel::{prelude::*, result::Error};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn get_all(conn: &db::DBConnection) -> Vec<User> {
        use crate::schema::users::dsl::users;
        users.load(conn).unwrap()
    }

    pub fn get_by_id(conn: &db::DBConnection, id: i32) -> Option<User> {
        use crate::schema::users::dsl::users;

        match users.find(id).first(conn) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub fn get_by_name(conn: &db::DBConnection, user_name: &str) -> Option<User> {
        use crate::schema::users::dsl::{name, users};

        match users.filter(name.eq(user_name)).first(conn) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub fn delete(conn: &db::DBConnection, id: i32) -> Result<(), Error> {
        use crate::schema::users::dsl::users;
        conn.transaction(|| diesel::delete(users.find(id)).execute(conn).map(|_| ()))
    }
}

#[derive(Debug, Clone, Insertable, Default, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn save(&self, conn: &db::DBConnection) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        conn.transaction(|| {
            diesel::insert_into(users)
                .values(&*self)
                .get_result::<User>(conn)
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserFactory {
    pub inner: NewUser,
}

impl UserFactory {
    pub fn name(mut self, name: &str) -> Self {
        self.inner.name = name.to_string();
        self
    }
}

impl Factory<User, NewUser> for UserFactory {
    fn new() -> Self {
        Self {
            inner: NewUser {
                name: format!("user-{}", random_string(16)),
            },
        }
    }

    fn build(&self) -> User {
        User {
            id: -1,
            name: self.inner.name.clone(),
        }
    }

    fn create(&self, conn: &db::DBConnection) -> User {
        self.inner
            .save(conn)
            .expect("Failed to create user with UserFactory")
    }
}

fn random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
