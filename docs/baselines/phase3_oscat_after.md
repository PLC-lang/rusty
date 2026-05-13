# Phase 3 baseline — oscat single-file `--check` after delta-aware participants

Captured with `PLC_INCR_TIMING=1 plc --check oscat.st stubs.st` after the
six lowering participants (Polymorphism × 2, Aggregate, Inheritance,
Retain, Array) and PropertyLowerer were migrated to skip their implicit
re-index / re-annotate when they had no work to do.

## Comparison

| Phase | parse | index (driver) | annotate (driver) | post_annotate participants | total pre-codegen |
|---|---:|---:|---:|---:|---:|
| Phase 0 (baseline) | 25.7 ms | 47.6 ms | 615.2 ms | 475.6 ms | ~688 ms |
| Phase 3 | 19.6 ms | 29.7 ms | **390.0 ms** | **256.1 ms** | ~439 ms |
| **Saved** | – | ~18 ms | **~215 ms (-35 %)** | **~220 ms (-46 %)** | **~250 ms (-36 %)** |

## Per-participant impact

The wins are dominated by the participants that did a full whole-project
re-annotate on every build:

| Participant | Phase 0 | Phase 3 | Why |
|---|---:|---:|---|
| `PropertyLowerer::post_annotate` | 113 ms | **164 ns** | oscat has no properties; precheck on `Index::has_any_properties()` is exact |
| `InheritanceLowerer::post_annotate` | 115 ms | **24 µs** | oscat has no super-class / interface declarations |
| `ArrayLowerer::pre_annotate` | 13 ms | 1.2 ms | oscat has no array-literal assignments; `lower_literal_arrays` reports `false` |
| `RetainParticipant::post_index` | 12.3 ms | 5.9 ms | oscat has no `RETAIN`; `lower_retain` reports `false` |
| `PolymorphismLowerer::post_index` | 10 ms | 11.6 ms | oscat has classes / FBs; `table()` still emits → re-index still runs |
| `PolymorphismLowerer::post_annotate` | 128 ms | 135 ms | oscat has polymorphism → dispatch still runs → re-index + re-annotate still fire |
| `AggregateTypeLowerer::post_annotate` | 121 ms | 120 ms | oscat has aggregate return types → visit still rewrites → re-index + re-annotate still fire |

The three remaining big-cost participants
(`PolymorphismLowerer::post_annotate`, `AggregateTypeLowerer::post_annotate`,
and `PolymorphismLowerer::post_index`) are doing real work — they're not
no-ops the way the others are on oscat. Eliminating their full re-passes
needs richer per-unit deltas: the lowerer reporting *which* units it
mutated so the pipeline can re-index/re-annotate just those, not the
whole project. That's the Phase 4+ work.

## Reproducing

```sh
cargo build --release -p plc_driver
cd .baseline/oscat
PLC_INCR_TIMING=1 \
  ../../target/release/plc --check oscat.st stubs.st 2>&1 | head -70
```

Numbers vary ±10% run-to-run from CPU noise; capture three runs and
compare medians when claiming a follow-up win.
