APP_CONTAINER_NAME=rust

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new hello_cargo

build/dev:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd hello_cargo/src && cargo build"

build/prod:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd hello_cargo/src && cargo build --release"

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd hello_cargo/src && cargo run"

check:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd hello_cargo/src && cargo check"
