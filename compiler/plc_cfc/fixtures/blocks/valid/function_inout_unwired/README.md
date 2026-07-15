What: `addInto` with its `VAR_IN_OUT acc` left unwired. Every parameter must
still be supplied, so the in_out is emitted as an empty argument (`acc := `).
Transpile-only: an in_out cannot be empty, so the main pipeline rejects it with
E031 (an in_out arg must be a reference) — same stance as a negated in_out, we
emit faithfully rather than pre-validate.

Illustrated:
```
          addInto (0)
        +--------------------+
  5 --> | delta      addInto | --> result (1)
   (unwired) acc             |
        +--------------------+
```
