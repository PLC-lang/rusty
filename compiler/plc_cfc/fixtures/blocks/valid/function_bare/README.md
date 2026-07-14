What: `myAdd` wired at its inputs but with every output pin unread. Nothing
consumes it, so no temporaries are generated; the return is discarded and the
unread output is passed as an empty argument (`myAddDoubled => `) — a function
must still receive every parameter.

Illustrated:
```
        myAdd (0)
      +--------------------+
a --> | in1          myAdd |
b --> | in2   myAddDoubled |
      +--------------------+
      (no outputs consumed)
```
