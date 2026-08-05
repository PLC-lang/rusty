# variadic_feedback

What: The builtin variadic `ADD` as an accumulator — its output feeds back
into its *first* pin, the second pin reads a concrete `seed : DINT`. The
captured temporary starts out generic (`__ADD__T`); the concrete input must
decide `DINT`, no matter that the generic operand comes first when the
annotator folds the operand types (an unresolved generic loses against any
concrete type in `get_bigger_type` — it must never promote the pair to REAL).

Illustrated:
```
          ADD (0)
        +----------------+
   .--> | IN1        ADD | --+--> acc (1)
seed--> |                |   |
        +----------------+   |
   '-------------------------'
```
