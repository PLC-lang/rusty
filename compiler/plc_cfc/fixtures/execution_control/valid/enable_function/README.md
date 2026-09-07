What: a stateless (function) call with execution control. The whole captured
call, including the `__out_...` return capture, must sit inside the EN guard so
the temporary keeps its previous value while EN is FALSE. ENO feeds the `done`
sink and resolves transparently to `trigger`.

Illustrated:
```
                  myAdd (0)
            +--------------------+
trigger --> | EN             ENO | --> done (2)
      a --> | in1          myAdd | --> sum (1)
      b --> | in2   myAddDoubled |
            +--------------------+
            (myAddDoubled unread)
```
