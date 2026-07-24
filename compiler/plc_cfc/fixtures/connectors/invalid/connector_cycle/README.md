What: Connector `x`'s input is wired to its own continuation's output, so the
label routing loops back on itself; a consumed read chases the cycle and can
never reach a source (E082).

Illustrated:
```
        +-------------------+
        v                   |
    >x> --> bar (0)     x< -+   (continuation feeds both the sink and its own connector)
```
