This is a repo for reproducing Diesel's test transaction panicing when ran in async context.

## Create DB

``` sh
psql> create database async_dev;
psql> create database async_test;
```

## Run tests

You should see tests failing randomly at `begin_test_transaction` in the controllers tests.
```
$ make test
DATABASE_URL="postgres://postgres:@localhost:5432/async_test" diesel setup
DATABASE_URL="postgres://postgres:@localhost:5432/async_test" cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running target/debug/deps/asynctest-351e50735a0fa6ea

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/async_test_panic-5ae96e31346bcff7

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/integration-5e3c641ffc15416e

running 16 tests
test controllers::users::show::show_a_user ... ok
test controllers::users::show::show_a_user2 ... ok
test controllers::users::create::create_new_user1 ... ok
test controllers::users::list::return_list_of_users5 ... ok
test controllers::users::list::return_list_of_users ... FAILED
test controllers::users::create::create_new_user3 ... ok
test controllers::users::show::show_a_user3 ... ok
test controllers::users::create::create_new_user ... ok
test controllers::users::create::create_new_user2 ... ok
test controllers::users::list::return_list_of_users2 ... ok
test controllers::users::list::return_list_of_users4 ... ok
test controllers::users::show::show_a_user4 ... FAILED
test controllers::users::create::create_new_user4 ... FAILED
test models::user::get_by_id ... ok
test models::user::get_by_name ... ok
test controllers::users::list::return_list_of_users3 ... FAILED

failures:

---- controllers::users::list::return_list_of_users stdout ----
thread 'controllers::users::list::return_list_of_users' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', /home/darwin/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/connection/mod.rs:118:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- controllers::users::show::show_a_user4 stdout ----
thread 'controllers::users::show::show_a_user4' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', /home/darwin/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/connection/mod.rs:118:9

---- controllers::users::create::create_new_user4 stdout ----
thread 'controllers::users::create::create_new_user4' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', /home/darwin/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/connection/mod.rs:118:9

---- controllers::users::list::return_list_of_users3 stdout ----
thread 'controllers::users::list::return_list_of_users3' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', /home/darwin/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.4.6/src/connection/mod.rs:118:9


failures:
    controllers::users::create::create_new_user4
    controllers::users::list::return_list_of_users
    controllers::users::list::return_list_of_users3
    controllers::users::show::show_a_user4

test result: FAILED. 12 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

error: test failed, to rerun pass '--test integration'
make: *** [Makefile:4: test] Error 101
```
