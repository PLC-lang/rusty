What: the debug line of a diagram element. Each ordered element is located in
the diagram's own debug file `execution_order_line.cfc.execution_order_line`
at its execution order as the line, so a debugger that reads line N as order N
stops on the element the IDE shows. The store of each sink is its key
instruction, the only place a debugger stops.

Illustrated:
```
a --+--> b (1)
    +--> c (2)
```
