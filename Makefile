.PHONY: run lint test container

run:
	cargo run

check:
	cargo check

lint:
	cargo fmt --all && cargo clippy --workspace

test:
	cargo test --workspace --no-fail-fast && ./scripts/build.sh --lit

container:
	docker build -t rusty-dev .devcontainer/
	docker run -it --entrypoint bash -v $(PWD):/workspace -w /workspace rusty-dev
