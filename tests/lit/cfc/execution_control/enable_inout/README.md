What: EN guards a function with a VAR_IN_OUT parameter. While EN is FALSE the
call is skipped entirely, so the inout variable is neither evaluated nor
written back; a TRUE cycle transfers it once, and dropping EN holds both the
inout and the captured return value.

Illustrated:
```
               addInto (0)
            +---------------------+
trigger --> | EN                  |
      5 --> | delta       addInto | --> result (1)
 total <--> | acc                 |
            +---------------------+
```
