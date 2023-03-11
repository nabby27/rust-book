APP_CONTAINER_NAME=rust
PROJECT_NAME_1=vector
PROJECT_NAME_2=string
PROJECT_NAME_3=hashmap

new-project/vector:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_1)

new-project/string:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_2)

new-project/hashmap:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_3)

run/vector:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo run"

run/string:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo run"

run/hashmap:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_3)/src && cargo run"
