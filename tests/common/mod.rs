extern crate asynctest;

// pub(crate) mod convert;

use std::env;
use std::sync::Once;

use asynctest::config::database;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    static ref INIT: Once = Once::new();
}

// pub const TEST_TX_ERR_MSG: &str = "Failed to start test transaction";

pub fn setup() {
    INIT.call_once(|| {
        dotenv().ok();

        database::init();
    });
}
