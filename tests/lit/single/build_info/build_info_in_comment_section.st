// REQUIRES: system-linux

// End-to-end: a `.ident` assembler directive emitted at module level
// produces a `plc version …` entry in the ELF `.comment` section of the
// compiled object. Runs on every Linux CI lane (x86_64 and aarch64) to
// confirm the directive survives both backends — the metadata-based path
// did not on aarch64 (see #1720).

// Default emission: the line is present.
// RUN: %COMPILE -c %s && readelf -p .comment %T/%basename_t.out | %CHECK %s

// `--fno-ident`: the line is suppressed (still emits a valid object).
// Reuses the same output path the first RUN used (overwrites it); lit
// executes RUN lines sequentially so the first check is already done.
// RUN: %COMPILE --fno-ident -c %s && readelf -p .comment %T/%basename_t.out | FileCheck-21 --check-prefix=NOIDENT --allow-empty %s

FUNCTION main : DINT
    // CHECK: {{.*plc version.*}}
    // NOIDENT-NOT: plc version
    main := 0;
END_FUNCTION
