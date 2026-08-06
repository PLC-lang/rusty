What: a literal seeds `src`, which then fans out into two sinks; one
`ConnectionPointOut` is referenced by multiple wires. `main` checks both sinks
received the value (`x = 5, y = 5`).

Illustrated:
```
5 --> src (0)
src --+--> x (1)
      +--> y (2)
```
