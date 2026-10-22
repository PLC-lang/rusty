What: A program block `counter` called with input `in` from `countIn` (1); its
`out` pin is read into `countOut` (2). The call carries only the input; the
output is consumed as a member access on the program's global.

Illustrated:
```
              counter (1)
            +------------+
countIn --> | in     out | --> countOut (2)
            +------------+
```
