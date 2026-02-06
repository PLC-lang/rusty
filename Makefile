.PHONY: test container

test:
	cargo nextest run --workspace && ./scripts/build.sh --lit

container:
	docker build -t rusty-dev .devcontainer/
	docker run -it --entrypoint bash -v $(PWD):/workspace -w /workspace rusty-dev
