# Profiling Build Phases

The compiler driver has a built-in phase timer that records wall-clock
time for each stage of `BuildPipeline` and for every participant
invocation. It is intended for ad-hoc performance work — for example,
when investigating why a particular project compiles slower than
expected, or when measuring the impact of a change to the lowering
pipeline.

## Enabling

Set the environment variable `PLC_TIMING=1` before invoking the
compiler:

```sh
PLC_TIMING=1 plc --check my_project.st
```

When the variable is unset (or set to `0` / empty), the timers compile
to a no-op and emit nothing. There is no behavioural difference and no
test impact, so it is safe to leave the code path always present.

## Reading the output

Each timed scope writes one line to stderr on completion, indented by
nesting depth. Children appear *before* their parent's log line (the
parent prints when it drops, i.e. at end-of-scope), which matches the
standard flame-graph convention.

A condensed example from compiling a small project:

```text
[plc-timing] parse: 25.7ms
[plc-timing]   pre_index (participants): 12.6ms
[plc-timing]     pre_index/LoopDesugarer: 6.6ms
[plc-timing]     pre_index/ControlStatementParticipant: 3.0ms
[plc-timing]   ParsedProject::index: 7.4ms
[plc-timing]   post_index (participants): 27.6ms
[plc-timing]     post_index/PolymorphismLowerer: 11.7ms
[plc-timing]       ParsedProject::index: 9.6ms
[plc-timing]     post_index/RetainParticipant: 15.8ms
[plc-timing]       ParsedProject::index: 5.8ms
[plc-timing] index (driver): 47.6ms
[plc-timing] annotate (driver): 615.2ms
```

A few things to note from this trace:

- The outer `parse`, `index (driver)`, `annotate (driver)`, and
  `generate (driver)` scopes correspond to the four top-level phases.
- Each participant invocation is timed individually with a label of the
  form `<hook>/<participant-type-name>`, e.g. `post_index/PolymorphismLowerer`.
- `ParsedProject::index` and `IndexedProject::annotate` self-time, so
  any participant that re-invokes them appears with a nested
  `ParsedProject::index` (or `IndexedProject::annotate`) child line.
  Those nested re-passes are the main thing to look for when
  investigating slow builds.

## What to look for

The trace is most useful for spotting **redundant whole-project
re-passes**: cases where a participant mutates the AST and then re-runs
indexing or annotation against the entire project, even though only a
few units were touched. A nested `ParsedProject::index` or
`IndexedProject::annotate` under a participant hook is a visible
indicator of one of those re-passes.

## Adding new timed scopes

To time a new scope, construct a `PhaseTimer` and let it drop at the
end of the scope you want to measure:

```rust
use crate::pipelines::timing::PhaseTimer;

fn expensive_thing() {
    let _t = PhaseTimer::new("expensive_thing");
    // ... work ...
}
```

For participant-style instrumentation, prefer wrapping the *callee*
(the inner method that does the work) rather than each call site. That
way a re-entrant call from a participant gets timed automatically,
without having to thread timer code through every place that might
call into the method.

The timer label is the only argument and accepts any
`Into<String>`. Use a stable, easily-greppable string — these strings
end up in the trace output and may be parsed by tooling.
