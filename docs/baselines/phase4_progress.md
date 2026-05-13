# Phase 3 / Phase 4 wall-clock comparison

Median wall-clock times for `parse → index → annotate` on representative
projects, captured with `PLC_INCR_TIMING=1 plc --check`, release builds,
3 runs each.

Compared binaries:

| Label              | Commit       | What's in it |
|--------------------|--------------|---|
| pre Phase 3        | `23a3bb89a9` | Phase 0 timing only — original participant chain |
| post Phase 3       | `35b3b212b9` | Skip the participant re-pass when there's nothing to do (precheck / bool) |
| post Phase 4 (partial) | HEAD     | Per-unit re-index inside `PolymorphismLowerer::post_index` |

## Multi-file oscat (556 source files, one POU per file)

Files split from upstream `plc-lang/oscat` HEAD on POU boundaries (one
`FUNCTION` / `FUNCTION_BLOCK` / `PROGRAM` / `CLASS` / `TYPE` per file),
written to `.baseline/oscat-multi/src/*.st` (gitignored). Mirrors the
historically-common "one POU per unit" style of real PLC projects.

| Phase                  | parse | index (driver) | annotate (driver) | total pre-codegen |
|------------------------|------:|---------------:|------------------:|------------------:|
| pre Phase 3            | 21 ms | 25 ms          | **252 ms** (med)  | ~298 ms |
| post Phase 3           | 20 ms | 23 ms          | **162 ms** (med)  | ~205 ms |
| post Phase 4 (partial) | 20 ms | 21 ms          | **160 ms** (med)  | ~201 ms |

Phase 3 saves **~90 ms on `annotate` (-36 %)** by skipping
`PropertyLowerer` and `InheritanceLowerer` re-passes (oscat has
neither) plus smaller skips. Phase 4 partial adds only ~2 ms on top
because oscat's annotate phase is dominated by participants that
genuinely have work to do (`PolymorphismLowerer::post_annotate`,
`AggregateTypeLowerer::post_annotate`) — those still re-annotate the
whole project. Phase 4 partial does shave ~2 ms off `index` from the
per-unit re-index in `PolymorphismLowerer::post_index`.

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

Multi-file oscat's `annotate` phase is still ~160 ms dominated by:

- `PolymorphismLowerer::post_annotate` — dispatch lowering, still
  re-annotates the whole project.
- `AggregateTypeLowerer::post_annotate` — aggregate-return rewriting,
  still re-indexes + re-annotates the whole project.

Reducing those needs the lowerer to report *which units* it mutated
(extending `lower_retain`-style bool to `Vec<UnitId>`-style delta), so
the pipeline can re-annotate only those units plus the reverse-dep
closure built in Phase 2. That's the next slice of Phase 4.
