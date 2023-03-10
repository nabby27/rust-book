APP_CONTAINER_NAME=rust
PROJECT_NAME=backyard
LIBPROJECT_NAME=restaurant

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME)

new-library:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBPROJECT_NAME) --lib

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo run"

