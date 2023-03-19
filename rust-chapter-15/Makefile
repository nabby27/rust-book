APP_CONTAINER_NAME=rust
PROJECT_NAME=generic_box

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cargo new $(PROJECT_NAME)"

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(PROJECT_NAME)/src && cargo run"
