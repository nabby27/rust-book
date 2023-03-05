APP_CONTAINER_NAME=rust
PROJECT_NAME=ownership

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME)

build/dev:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo build"

build/prod:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo build --release"

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo run"

check:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo check"

update:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo check"

# Only Linux and Mac can open automatically the documentation on browser
doc:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo doc --open"
	@open ./$(PROJECT_NAME)/target/doc/$(PROJECT_NAME)/index.html
