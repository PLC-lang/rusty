# Phase 3 / Phase 4 wall-clock comparison

Median wall-clock times for `parse → index → annotate` on representative
projects, captured with `PLC_INCR_TIMING=1 plc --check`, release builds,
3 runs each.

Compared binaries:

| Label              | Commit       | What's in it |
|--------------------|--------------|---|
| pre Phase 3        | `23a3bb89a9` | Phase 0 timing only — original participant chain |
| post Phase 3       | `35b3b212b9` | Skip the participant re-pass when there's nothing to do (precheck / bool) |
| post Phase 4 step 1 | `4cca9fdeb5` | Per-unit re-index inside `PolymorphismLowerer::post_index` |
| post Phase 4 step 2 | HEAD     | Plus per-unit re-index + reverse-dep-closure partial re-annotate inside `AggregateTypeLowerer::post_annotate` |

## Multi-file oscat (556 source files, one POU per file)

Files split from upstream `plc-lang/oscat` HEAD on POU boundaries (one
`FUNCTION` / `FUNCTION_BLOCK` / `PROGRAM` / `CLASS` / `TYPE` per file),
written to `.baseline/oscat-multi/src/*.st` (gitignored). Mirrors the
historically-common "one POU per unit" style of real PLC projects.

| Phase                   | parse | index (driver) | annotate (driver) | total pre-codegen |
|-------------------------|------:|---------------:|------------------:|------------------:|
| pre Phase 3             | 21 ms | 25 ms          | **252 ms** (med)  | ~298 ms |
| post Phase 3            | 20 ms | 23 ms          | **162 ms** (med)  | ~205 ms |
| post Phase 4 step 1     | 20 ms | 21 ms          | **160 ms** (med)  | ~201 ms |
| post Phase 4 step 2     | 20 ms | 22 ms          | **152 ms** (med)  | ~194 ms |

Phase 3 saves **~90 ms on `annotate` (-36 %)** by skipping
`PropertyLowerer` and `InheritanceLowerer` re-passes (oscat has
neither) plus smaller skips. Phase 4 step 1 cut another ~2 ms by
making `PolymorphismLowerer::post_index` re-index only the units
whose vtables it actually emitted. Phase 4 step 2 cut another ~8 ms
by giving `AggregateTypeLowerer::post_annotate` the same treatment
plus partial (parallel) re-annotation of the reverse-dependency
closure — most of the closure on oscat is "everything that calls a
STRING-returning function", which is a wide set, but the parallel
path is still faster than the unconditional full re-annotate.

## Multi-file polymorphism test (3 source files: base / derived / main)

Located at `tests/lit/multi/incremental_polymorphism_p4/`. Designed to
exercise cross-unit vtable generation and dispatch lowering:

- `src/base.st`: declares `FUNCTION_BLOCK Base` with a virtual method.
- `src/derived.st`: declares `FUNCTION_BLOCK Derived EXTENDS Base`,
  overrides the method.
- `main.st`: dispatches through `POINTER TO Base` to each instance.

| Phase                  | index (driver) | annotate (driver) |
|------------------------|---------------:|------------------:|
| pre Phase 3            | 1.0 ms (med)   | 3.4 ms (med)      |
| post Phase 3           | 0.86 ms (med)  | 2.3 ms (med)      |
| post Phase 4 (partial) | **0.52 ms** (med) | 2.1 ms (med)   |

Phase 4 partial cuts index time by **~40 %** vs. Phase 3 on this test.
The win comes specifically from per-unit re-indexing inside
`PolymorphismLowerer::post_index`: previously, after vtable generation,
the pipeline re-indexed the entire project (3 units) to pick up the new
`__vtable_*` types; now it re-indexes only the units that actually got
a new vtable definition. With 3 units this is a small absolute saving;
with 556 oscat-style units it would scale.

## Methodology notes

- Median of 3 runs per cell. Runs differ ±5–10 % from CPU noise; the
  structural differences between phases are larger.
- `parse` is unchanged across all phases (no parser code touched);
  small variations come from filesystem cache and CPU jitter.
- The `pre Phase 3` binary doesn't include `Index::has_any_properties`
  or the precheck helpers — its baseline reflects the legacy
  unconditional re-pass behaviour.
- `oscat-multi/src/` is gitignored. Reproduce by running the splitter
  in `docs/baselines/oscat_multi_split.py` (or by inlining the Python
  snippet documented in this file's git history).

## What Phase 4 still owes

Multi-file oscat's `annotate` phase is still ~150 ms dominated by:

- `PolymorphismLowerer::post_annotate` — dispatch lowering, still
  re-indexes + re-annotates the whole project. The interface- and
  pou-dispatch visitors don't track per-unit mutation today; making
  them do so is the next slice.
- `RetainParticipant::post_index` and `ArrayLowerer::pre_annotate` —
  both return `bool` from Phase 3. Trivial upgrades to `Vec<usize>`
  would let them use the per-unit re-index path; the win is small on
  oscat because the bool already short-circuits the empty case.

These are increments rather than headline rewrites.
