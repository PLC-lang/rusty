# previous_cycle_read

Networks where a **block's result is consumed before the block is evaluated**. CFC executes
cyclically — conceptually `WHILE TRUE DO <network> END_WHILE` — so such a consumer is legal: it
reads the value the block produced in the **previous cycle** (the type's default value on the very
first cycle). This is how feedback loops, integrators and filters are drawn.

The transpiler makes this work by creating every block-result variable *up front*, in a plain `VAR`
block that persists across cycles; the consumer then simply reads it before the block overwrites it
at its own slot. No diagnostic is reported. Note that inside a **FUNCTION** body the same wiring
still transpiles and compiles, but reads the type's initial value on every call — a function's
storage is re-initialized per invocation, so a "previous" value never exists.

- `alwaysFive.st`, `isReady.st`, `square.st` — the functions being called; their content is
  irrelevant, only their results' wiring matters.

## `sink.cfc` — variable reads a result from the previous cycle

The sink runs at priority `(0)`, before the block producing its value at `(1)`:

```text
   +-- alwaysFive --+ (1)
   |      alwaysFive|------->  result  (0)
   +----------------+

   (n)   evaluation-priority badges shown by the IDE
```

```text
result := __alwaysFive_res_0;
__alwaysFive_res_0 := alwaysFive();
```

## `conditional_return.cfc` — return guards on a previous-cycle result

The conditional return at `(0)` fires on the condition the network produced last cycle:

```text
   +--- isReady ----+ (1)
   |         isReady|------->| RETURN |  (0)
   +----------------+
```

```text
IF __isReady_res_0 THEN RETURN; END_IF;
__isReady_res_0 := isReady();
```

Note that when the return fires, the rest of the network is skipped and every result variable keeps
its value until the next full cycle.

## `block_argument.cfc` — block computes on a previous-cycle result

`square` runs at `(0)` on the argument `alwaysFive` produces at `(1)`; the sink at `(2)` is
in-order:

```text
   +-- alwaysFive --+ (1)      +---- square ----+ (0)
   |      alwaysFive|--------->| x        square|------->  result  (2)
   +----------------+          +----------------+
```

```text
__square_res_0 := square(x := __alwaysFive_res_1);
__alwaysFive_res_1 := alwaysFive();
result := __square_res_0;
```

## `alias.cfc` — previous-cycle read through a connector/continuation pair

Same as `sink.cfc`, but the wire is routed through a connector/continuation pair; the alias
resolution is transparent to the ordering:

```text
   +-- alwaysFive --+ (1)
   |      alwaysFive|------->[ Connector "relay" ]
   +----------------+

                      [ Continuation "relay" ]------->  result  (0)
```

See the `previous_cycle_*` tests in `transpiler.rs`.
