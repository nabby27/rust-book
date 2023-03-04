APP_CONTAINER_NAME=rust
PROJECT_NAME_1=variables
PROJECT_NAME_2=functions
PROJECT_NAME_3=branches
PROJECT_NAME_4=loops

new-project/variables:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_1)

new-project/functions:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_2)

new-project/branches:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_3)

new-project/loops:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME_4)

build/dev/variables:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo build"

build/dev/functions:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo build"

build/dev/branches:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_3)/src && cargo build"

build/dev/loops:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_4)/src && cargo build"

run/variables:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_1)/src && cargo run"

run/functions:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_2)/src && cargo run"

run/branches:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_3)/src && cargo run"

run/loops:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME_4)/src && cargo run"
