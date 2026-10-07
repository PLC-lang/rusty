What: A block output routed through a connector/continuation pair: `counter`'s
`out` pin (1) feeds connector `x`, whose continuation feeds the sink `result`
(2). The trace hops the pair and still lands on the block's output pin, so the
read is a member access, not a plain source.

Illustrated:
```
seed --> in [counter] out --> >x>    x> --> result (2)
             (1)
```
