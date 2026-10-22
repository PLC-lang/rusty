What: A block calls `counter`, but no such POU is declared anywhere in the
project (deliberately no companion `.st` here), so the call cannot be
classified and is rejected (E146).

Illustrated:
```
              counter (1)
            +------------+
countIn --> | in     out | --> countOut (2)
            +------------+
            (unknown type: no POU named counter)
```
