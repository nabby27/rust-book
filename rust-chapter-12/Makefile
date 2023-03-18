APP_CONTAINER_NAME=rust
PROJECT_NAME=minigrep

new-project:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(PROJECT_NAME)

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo run -- $(text) $(file)"

run/redirect:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo run -- $(text) $(file) > output.txt"

test:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_NAME)/src && cargo test $(name)"
