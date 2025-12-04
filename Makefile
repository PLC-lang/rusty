.PHONY: test report status-debug

test:
	@clear
	cargo nextest run --no-fail-fast --workspace

report:
	@clear
	@cargo nextest run --no-fail-fast --workspace 2>&1 | \
		awk "/thread '.*' \\([0-9]+\\) panicked at/ { \
			match(\$$0, /thread '[^']+'/); \
			test = substr(\$$0, RSTART+8, RLENGTH-9); \
			match(\$$0, /panicked at [^:]+:[0-9]+:[0-9]+/); \
			loc = substr(\$$0, RSTART, RLENGTH); \
			count[loc]++; \
			total++; \
			if (tests[loc] == \"\") tests[loc] = test; \
			else if (split(tests[loc], arr, \"|\") < 10) tests[loc] = tests[loc] \"|\" test; \
		} END { \
			for (k in count) print count[k] \"\t\" k \"\t\" tests[k]; \
			print \"TOTAL\t\" total; \
		}" | sort -rn | while IFS=$$'\t' read cnt loc names; do \
			if [ "$$cnt" = "TOTAL" ]; then \
				echo ""; \
				echo "========================================"; \
				echo "Total failed tests: $$loc"; \
			else \
				echo ""; \
				echo "$$cnt $$loc"; \
				echo "$$names" | tr '|' '\n' | sed 's/^/    /'; \
			fi; \
		done
