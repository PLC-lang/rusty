What: EN wired from another block's output pin instead of a plain variable.
The guard reads the producing instance's member (`g.ok`), which is side-effect
free like any block-output read.

Illustrated:
```
  g : gate (1)          inst : counter (2)
+-------------+        +-------------------+
|          ok | -----> | EN                |
+-------------+        |                   |
           localIn --> | in            out | --> localOut (3)
                       +-------------------+
```
