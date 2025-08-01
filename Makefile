.PHONY: test run

test:
	cargo nextest run --workspace --no-fail-fast && ./scripts/build.sh --lit

run:
	cargo r -- target/demo.st tests/lit/util/printf.pli --linker=clang && ./demo.st.out
