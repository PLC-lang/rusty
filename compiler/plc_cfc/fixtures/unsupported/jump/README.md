# jump (unsupported)

A network containing a **jump** object. Jumps are a valid part of the CFC model and
deserialize fine, but lowering them is not implemented yet, so the resolver rejects
any network that contains one with an `unimplemented!("CFC jumps are not yet
supported")` panic rather than silently dropping the jump (which would change the
POU's control flow with no diagnostic).

```text
   enable  ------>| JMP skip |  (0)

   input   ------>  result      (1)

   JMP skip   an (unsupported) conditional jump to network label "skip"
   (0),(1)    evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test; its network holds a `ppx:Jump` object.

This fixture lives under `fixtures/unsupported/` (rather than `valid/` or `invalid/`)
because the file itself is well-formed — the feature is simply not supported yet. See
the `jump_is_unsupported` tests in `resolver.rs` and `transpiler.rs`.
