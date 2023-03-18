APP_CONTAINER_NAME=rust
LIBRARY_NAME_1=my_crate
LIBRARY_NAME_2=art

new-library/my_crate:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBRARY_NAME_1) --lib

run/my_crate:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_1) && cargo run"

test/my_crate:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_1) && cargo test"

doc/my_crate:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_1) && cargo doc"

new-library/art:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) cargo new $(LIBRARY_NAME_2) --lib

run/art:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_2) && cargo run"

test/art:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_2) && cargo test"

doc/art:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_2) && cargo doc"

publish/art:
	@docker compose run --rm --service-ports $(APP_CONTAINER_NAME) /bin/sh -c "cd $(LIBRARY_NAME_2) && cargo publish"
