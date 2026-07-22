# Fixtures

CFC example projects that double as transpiler/resolver test inputs. Every `.cfc`
here is real, IDE-exported-shaped PLCopen XML (the `ppx` namespace,
`www.iec.ch/public/TC65SC65BWG7TF10`) with valid `RelPosition`/`Size` values, so
it can be **copy-pasted / imported into the IDE unchanged**. The authoritative
schema is `src/model.rs` plus the verbatim exports under `reference/`.

## Layout

```
fixtures/
  valid/<name>/     compiles cleanly (warnings allowed)
  invalid/<name>/   rejected with a diagnostic
  reference/        verbatim IDE exports kept as schema ground-truth
```

Each fixture is a folder containing a `README.md` and the `.cfc` file(s) (plus
companion `.st` files when a case needs them). Name the `.cfc` after the fixture.

## Fixture README format

Lean: a `What:` description, then an `Illustrated:` ASCII sketch of the network
in a fenced block. Draw `source --> sink`, mark sink execution order with `(n)`,
and skip element borders — the names, arrows and order carry the meaning.
(Borders earn their place only later, for real multi-pin blocks.)

```
What: one or two sentences on what the network does and why it exists.

Illustrated:
​```
foo --+--> bar (0)
      +--> baz (1)
​```
```
