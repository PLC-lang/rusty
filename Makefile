demo:
	cargo r -- target/demo.st tests/lit/util/printf.pli --linker=clang && ./demo.st.out

ir:
	cargo r -- target/demo.st tests/lit/util/printf.pli --linker=clang --ir --output=/dev/stdout

test:
	cargo nextest run --no-fail-fast --workspace && ./scripts/build.sh --lit

.PHONY: demo ir test