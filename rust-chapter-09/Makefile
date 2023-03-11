APP_CONTAINER_NAME=rust
PROJECT_NAME=error_handling

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME)

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo run"
