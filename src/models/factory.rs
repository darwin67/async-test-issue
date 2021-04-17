use crate::config::database as db;
use std::default::Default;

pub trait Factory<T, N: Default> {
    fn new() -> Self;
    fn build(&self) -> T;
    fn create(&self, conn: &db::DBConnection) -> T;
}
