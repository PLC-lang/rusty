What: the debug info of a control-flow element. The conditional return (order 0)
and the assignment it guards (order 1) are located in the diagram's debug file
at their order plus one; the guard source has no order and is read at the
return's location. The return has a key instruction (a debugger stop) in each
of its blocks: the guard branch and the return behind it.

Illustrated:
```
guard --> RETURN (0)
42 --> result (1)
```
