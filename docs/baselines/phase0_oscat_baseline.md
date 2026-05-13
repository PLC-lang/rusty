# Phase 0 baseline — oscat single-file `--check`

Captured with `PLC_INCR_TIMING=1 plc --check oscat.st stubs.st` on the
`incremental_compilation` branch, release build. Source: upstream
`plc-lang/oscat` HEAD at the time of capture (single-file lib,
~30k lines).

Two consecutive runs included to show CPU noise — the *structure* of the
cost distribution is what matters, not absolute numbers.

The diagnostic errors emitted at the end (`Unknown type: TON / TOF`) are
expected: we ran without the `iec61131std` library that oscat references.
Errors fire during annotate, so the timing data above them is unaffected.

## Headline numbers (Run 1)

| Phase | Wall time | Notes |
|---|---:|---|
| parse | 25.7 ms | |
| index (driver) | 47.6 ms | |
| └ ParsedProject::index (first pass) | 7.4 ms | |
| └ post_index participants | 27.6 ms | of which PolymorphismLowerer 11.7 ms + RetainParticipant 15.8 ms ≈ 27 ms is **redundant re-indexes** |
| annotate (driver) | **615.2 ms** | dominated by participant re-passes |
| └ IndexedProject::annotate (first pass) | 108.3 ms | |
| └ pre_annotate participants | 31.1 ms | InitParticipant + ArrayLowerer each trigger a ParsedProject::index re-pass |
| └ post_annotate participants | **475.6 ms** | of which **four ~106 ms `IndexedProject::annotate` re-passes** = ~424 ms wasted |
| **total (pre-codegen)** | **~688 ms** | |

## Headline numbers (Run 2, for noise check)

| Phase | Wall time |
|---|---:|
| parse | 21.1 ms |
| index (driver) | 40.5 ms |
| annotate (driver) | 605.5 ms |
| └ post_annotate participants | 468.6 ms |

## The redundant re-passes, called out

Inside the participant chain, the following implicit whole-project
re-passes fire on every build (visible as nested timer lines in the
trace). Each row is the cost of one such re-pass on oscat:

| Source | Re-runs | Per-call cost |
|---|---|---:|
| `PolymorphismLowerer::post_index` | full `ParsedProject::index` | ~10 ms |
| `RetainParticipant::post_index` | full `ParsedProject::index` | ~12 ms |
| `InitParticipant::pre_annotate` | full `ParsedProject::index` | ~10 ms |
| `ArrayLowerer::pre_annotate` | full `ParsedProject::index` | ~10 ms |
| `LoopDesugarer::post_annotate` | full `IndexedProject::annotate` | ~105 ms |
| `PropertyLowerer::post_annotate` | full `IndexedProject::annotate` (and an inner index) | ~108 ms |
| `PolymorphismLowerer::post_annotate` | full `index` + full `annotate` | ~10 + ~102 ms |
| `AggregateTypeLowerer::post_annotate` | full `index` + full `annotate` | ~9 + ~104 ms |
| `InheritanceLowerer::post_annotate` | full `IndexedProject::annotate` | ~105 ms |

Reading off the trace: roughly **4 redundant whole-project re-annotates**
× ~106 ms = ~424 ms of wasted work inside `annotate (driver)` on every
build of oscat. The "headline near-term win" from Phase 3 is the
elimination of (most of) this.

## Methodology notes

- All timings from the same machine, release build.
- Numbers vary ±10% run-to-run due to CPU noise. Capture three runs and
  compare medians when claiming a Phase 3 speedup.
- `--check` skips codegen, isolating parse/index/annotate which is what
  Phase 3 affects.
- Single-file oscat is a useful upper bound for "how bad can the
  redundant re-pass cost get on a real project." A multi-file split (one
  POU per file) is deferred to Phase 3 verification: it lets us also
  measure how much of the re-annotate closure is actually justified vs
  wasted.

## Reproducing

```sh
git clone --depth 1 https://github.com/plc-lang/oscat .baseline/oscat
cargo build --release -p plc_driver
cd .baseline/oscat
PLC_INCR_TIMING=1 \
  ../../target/release/plc --check oscat.st stubs.st 2>&1 | head -90
```
