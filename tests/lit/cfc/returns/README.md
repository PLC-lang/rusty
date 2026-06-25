# returns

Integration test: CFC **conditional and unconditional returns**. Two CFC programs, each
a stack of priority-ordered rows; a `Return` between two `out` assignments decides whether
the second one is reached.

`Guard` — a *conditional* return (fires only when `skip` is true):

```text
   1     ------------->  out      (0)   out := 1
   skip  ------------->  RETURN   (1)   IF skip THEN RETURN; END_IF
   2     ------------->  out      (2)   out := 2
```

`Always` — an *unconditional* return (no wired condition; always fires):

```text
   1   --------------->  out      (0)   out := 1
                         RETURN    (1)   RETURN
   2   --------------->  out      (2)   out := 2  (dead — never reached)
```

- `guard.cfc` — `PROGRAM Guard` (`skip : BOOL`, `out : DINT`).
- `always.cfc` — `PROGRAM Always` (`out : DINT`).
- `main.st` — entry point: runs `Guard` with `skip` true then false, and `Always` once.

Rows execute in evaluation-priority order; the return short-circuits the rows below it.
So `Guard` yields `1` when `skip` is true and `2` otherwise, and `Always` always yields `1`.
