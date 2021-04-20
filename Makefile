.PHONY: test
test:
	APP_ENV=test DATABASE_URL="postgres://postgres:@localhost:5432/async_test" diesel setup
	APP_ENV=test DATABASE_URL="postgres://postgres:@localhost:5432/async_test" cargo test
