APP_CONTAINER_NAME=rust
PROJECT_NAME_1=generics
PROJECT_NAME_2=traits
PROJECT_NAME_3=lifetimes

new-project/generics:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_1)

new-project/traits:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_2)

new-project/lifetimes:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_3)

run/generics:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo run"

run/traits:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo run"

run/lifetimes:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_3)/src && cargo run"
