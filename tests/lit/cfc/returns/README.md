# returns

Integration test: CFC **conditional, unconditional, and negated returns**. Three CFC
programs, each a stack of priority-ordered rows; a `Return` between two `out` assignments
decides whether the second one is reached.

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

`GuardNot` — a *negated* conditional return (the condition wire is negated, so it fires
when `keep` is **false**):

```text
   1     ------------->  out       (0)   out := 1
   keep  ----(NOT)----->  RETURN   (1)   IF NOT keep THEN RETURN; END_IF
   2     ------------->  out       (2)   out := 2
```

- `guard.cfc` — `PROGRAM Guard` (`skip : BOOL`, `out : DINT`).
- `always.cfc` — `PROGRAM Always` (`out : DINT`).
- `guard_not.cfc` — `PROGRAM GuardNot` (`keep : BOOL`, `out : DINT`); the `Return` carries `negated="true"`.
- `main.st` — entry point: runs `Guard` (skip true then false), `Always` once, and `GuardNot` (keep true then false).

Rows execute in evaluation-priority order; the return short-circuits the rows below it.
So `Guard` yields `1` when `skip` is true and `2` otherwise; `Always` always yields `1`;
and `GuardNot` (negated) yields `2` when `keep` is true and `1` when `keep` is false.
