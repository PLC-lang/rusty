# evaluation_order (invalid)

Networks where a **block's result is consumed before the block is evaluated**. A block's
results only exist once the block itself has run, so every wire leaving a block must lead
to a consumer with a *strictly higher* evaluation priority. Each `.cfc` file here breaks
that rule with a different kind of consumer and must be rejected with **E142** — without
the validation, the transpiler would panic on all of them.

- `alwaysFive.st`, `isReady.st`, `square.st` — the functions being called; their content
  is irrelevant, only their results' wiring matters.

## `sink.cfc` — variable consumes a result too early

The sink runs at priority `(0)`, before the block producing its value at `(1)`:

```text
   +-- alwaysFive --+ (1)
   |      alwaysFive|------->  result  (0)
   +----------------+

   (n)   evaluation-priority badges shown by the IDE
```

## `conditional_return.cfc` — return condition consumes a result too early

The conditional return at `(0)` guards on a condition that is only produced at `(1)`:

```text
   +--- isReady ----+ (1)
   |         isReady|------->| RETURN |  (0)
   +----------------+
```

## `block_argument.cfc` — block argument consumes a result too early

`square` runs at `(0)` but its argument is produced by `alwaysFive` at `(1)`; the sink
at `(2)` is correctly ordered and must *not* be reported:

```text
   +-- alwaysFive --+ (1)      +---- square ----+ (0)
   |      alwaysFive|--------->| x        square|------->  result  (2)
   +----------------+          +----------------+
```

## `alias.cfc` — aliased result consumed too early

Same inversion as `sink.cfc`, but the wire is hidden behind a connector/continuation
pair; the validation must resolve the alias back to the real producer:

```text
   +-- alwaysFive --+ (1)
   |      alwaysFive|------->[ Connector "relay" ]
   +----------------+

                      [ Continuation "relay" ]------->  result  (0)
```
