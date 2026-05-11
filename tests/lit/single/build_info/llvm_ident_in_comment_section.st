// REQUIRES: system-linux

// Asserts that plc-emitted ELF objects carry a `plc version …` line in their
// `.comment` section by default, mirroring clang's / rustc's
// `llvm.ident`-driven self-identification convention. See #1720.

// Default emission: the line is present.
// RUN: %COMPILE -c %s && readelf -p .comment %T/%basename_t.out | %CHECK %s

// `--fno-ident`: the line is suppressed (still emits a valid object). Reuses
// the same output path the first RUN used (overwrites it); since lit executes
// RUN lines sequentially, the first check is already done.
// RUN: %COMPILE --fno-ident -c %s && readelf -p .comment %T/%basename_t.out | FileCheck-21 --check-prefix=NOIDENT --allow-empty %s

FUNCTION main : DINT
    // CHECK: {{.*plc version.*}}
    // NOIDENT-NOT: plc version
    main := 0;
END_FUNCTION
