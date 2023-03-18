APP_CONTAINER_NAME=rust
PROJECT_NAME_1=closures
PROJECT_NAME_2=iterators

new-project/closures:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_1)

new-project/iterators:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_2)

run/closures:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo run"

run/iterators:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo run"
