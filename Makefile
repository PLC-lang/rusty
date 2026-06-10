.PHONY: all fmt clippy test lit

all: fmt clippy test lit

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --tests -- -D warnings

test:
	cargo t --workspace

lit:
	./scripts/build.sh --lit
