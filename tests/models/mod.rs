use crate::common;
use asynctest::models::user::{User, UserFactory};
use asynctest::{config::database::DbPool, models::factory::Factory};
use diesel::prelude::*;
use diesel::result::Error;

#[test]
fn get_by_id() {
    common::setup();

    let conn = DbPool::conn();
    conn.test_transaction::<_, Error, _>(|| {
        let user = UserFactory::new().create(&conn);

        let result = User::get_by_id(&conn, user.id);
        assert!(result.is_some());

        if let Some(val) = result {
            assert_eq!(val.id, user.id);
        }

        Ok(())
    })
}

#[test]
fn get_by_name() {
    common::setup();

    let conn = DbPool::conn();
    conn.test_transaction::<_, Error, _>(|| {
        let user = UserFactory::new().name("dude").create(&conn);

        let result = User::get_by_name(&conn, "dude");
        assert!(result.is_some());

        if let Some(val) = result {
            assert_eq!(&val.name, &user.name);
        }

        Ok(())
    })
}
