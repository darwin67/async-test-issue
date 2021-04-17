use crate::common;
use actix_web::{http, web};
use asynctest::controllers::users::Endpoint;
use asynctest::models::user::UserFactory;
use asynctest::{
    config::database::{DbConn, DbPool},
    models::factory::Factory,
};
use common::convert::Convert;
use diesel::prelude::*;

mod list {
    use super::*;

    #[actix_rt::test]
    async fn return_list_of_users() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users2() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users3() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users4() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users5() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users6() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }

    #[actix_rt::test]
    async fn return_list_of_users7() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let mut users = vec![];
        for _ in 0..5 {
            users.push(UserFactory::new().create(&conn));
        }

        let result = Endpoint::list(DbConn::with(conn)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::OK);

            let list = resp.body().as_json();
            assert_eq!(list["data"].as_array().unwrap().len(), users.len());
        }
    }
}

mod create {
    use super::*;

    #[actix_rt::test]
    async fn create_new_user() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let data = UserFactory::new().name("hello").inner.clone();
        let result = Endpoint::create(DbConn::with(conn), web::Json(data)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::CREATED);
        }
    }

    #[actix_rt::test]
    async fn create_new_user1() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let data = UserFactory::new().name("hello").inner.clone();
        let result = Endpoint::create(DbConn::with(conn), web::Json(data)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::CREATED);
        }
    }

    #[actix_rt::test]
    async fn create_new_user2() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let data = UserFactory::new().name("hello").inner.clone();
        let result = Endpoint::create(DbConn::with(conn), web::Json(data)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::CREATED);
        }
    }

    #[actix_rt::test]
    async fn create_new_user3() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let data = UserFactory::new().name("hello").inner.clone();
        let result = Endpoint::create(DbConn::with(conn), web::Json(data)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::CREATED);
        }
    }

    #[actix_rt::test]
    async fn create_new_user4() {
        common::setup();

        let conn = DbPool::conn();
        conn.begin_test_transaction()
            .expect(common::TEST_TX_ERR_MSG);

        let data = UserFactory::new().name("hello").inner.clone();
        let result = Endpoint::create(DbConn::with(conn), web::Json(data)).await;
        assert!(result.is_ok());

        if let Ok(resp) = result {
            assert_eq!(resp.status(), http::StatusCode::CREATED);
        }
    }
}
