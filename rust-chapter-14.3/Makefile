APP_CONTAINER_NAME=rust
PROJECT_NAME=adder
LIBRARY_NAME_1=add_one
LIBRARY_NAME_2=art

build:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo build

test:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo test

new-project/adder:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME)

run/adder:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo run -p $(PROJECT_NAME)

new-library/add_one:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBRARY_NAME_1) --lib

test/add_one:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo test -p $(LIBRARY_NAME_1)

new-library/add_two:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBRARY_NAME_2) --lib
