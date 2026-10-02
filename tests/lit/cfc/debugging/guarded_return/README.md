What: the debug info of a control-flow element. The conditional return (order 1)
and the assignment it guards (order 2) are located in the diagram's debug file
at their order as the line; the guard source has no order and is read at the
return's location. The return has a key instruction (a debugger stop) in each
of its blocks: the guard branch and the return behind it.

Illustrated:
```
guard --> RETURN (1)
42 --> result (2)
```
