APP_CONTAINER_NAME=rust
PROJECT_NAME_1=structs
PROJECT_NAME_2=rectangles

new-project/structs:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_1)

new-project/rectangle:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_2)

run/structs:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo run"

run/rectangle:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo run"
