# Set the default target of this Makefile
.PHONY: all
all:: clippy test

.PHONY: check-features
check-features:
	cargo hack check --feature-powerset --no-dev-deps

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --doc

# Update vendored protos from mys-apis master branch.
.PHONY: update-protos
update-protos:
	@rm -rf vendored/mys-apis
	@rm -rf vendored/proto
	@git clone -q --depth=1 https://github.com/The-Social-Proof-Foundation/mys-apis.git vendored/mys-apis
	@mv vendored/mys-apis/proto vendored/
	@rm -rf vendored/mys-apis

# Update vendored protos from a local clone of mys-apis.
# Example invocation:
#     make -C crates/mys-rpc update-protos-local /path/to/mys-apis
.PHONY: update-protos-local
update-protos-local:
	@if [ -z "$(filter-out $@,$(MAKECMDGOALS))" ]; then \
		echo "Error: Path not provided. Usage: make update-protos-local /path/to/mys-apis"; \
		exit 1; \
	fi
	@if [ ! -d "$(filter-out $@,$(MAKECMDGOALS))/proto" ]; then \
		echo "Error: $(filter-out $@,$(MAKECMDGOALS))/proto does not exist"; \
		exit 1; \
	fi
	@rm -rf vendored/proto
	@cp -r "$(filter-out $@,$(MAKECMDGOALS))/proto" vendored/
	@echo "Updated protos from $(filter-out $@,$(MAKECMDGOALS))"

.PHONY: diff-protos
diff-protos:
	@rm -rf vendored/mys-apis
	@git clone -q --depth=1 https://github.com/The-Social-Proof-Foundation/mys-apis.git vendored/mys-apis
	git diff --no-index vendored/mys-apis/proto vendored/proto
	@rm -rf vendored/mys-apis

.PHONY: proto
proto:
	cargo run -p proto-build

%:
	$(MAKE) -C ../.. $@
