APP_CONTAINER_NAME=rust
LIBRARY_NAME=adder

new-library:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBRARY_NAME) --lib

run:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(LIBRARY_NAME)/src && cargo run"

test/parallel:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(LIBRARY_NAME)/src && cargo test $(name)"

test/consecutively:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(LIBRARY_NAME)/src && cargo test -- --test-threads=1"

test/ignored:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(LIBRARY_NAME)/src && cargo test -- --ignored"

test/include-ignored:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c " cd $(LIBRARY_NAME)/src && cargo test -- --include-ignored"
