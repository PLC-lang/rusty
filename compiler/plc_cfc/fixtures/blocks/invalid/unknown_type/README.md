What: A block calls `counter`, but no such POU is declared anywhere in the
project (deliberately no companion `.st` here), so the call cannot be
classified and is rejected (E146).

Illustrated:
```
countIn --> in [counter?] out (1) --> countOut (2)
```
