What: A program block `counter` called with input `in` from `countIn` (0); its
`out` pin is read into `countOut` (1). The call carries only the input; the
output is consumed as a member access on the program's global.

Illustrated:
```
countIn --> in [counter] out (0) --> countOut (1)
```
