# generic_unresolved

What: A generic block fed by nothing but its own feedback — both inputs of
`myGenAdd<T: ANY_NUM>` wire back to its return pin, so no external candidate
ever decides `T`. The monomorphization loop quiesces with the temporary still
generic and reports it instead of silently picking a type.

Illustrated:
```
          myGenAdd (0)
        +-----------------+
   .--> | a      myGenAdd | --+--> acc (1)
   +--> | b               |   |
   |    +-----------------+   |
   '--------------------------'   [return feeds back into a and b]
```
