.PHONY: test
test:
	DATABASE_URL="postgres://postgres:@localhost:5432/async_test" diesel setup
	DATABASE_URL="postgres://postgres:@localhost:5432/async_test" cargo test
