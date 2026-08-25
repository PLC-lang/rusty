What: a four-block chain that alternates gated and ungated blocks, with the two
gates toggled in sync and out of sync. Ungated blocks run every cycle and
consume whatever their producer holds, so a disabled head feeds its buffered
value downstream, and a disabled tail freezes while the head keeps advancing.

Illustrated:
```
          c : counter (0)      d1 : doubler (1)     p : addOne (2)       d2 : doubler (3)
        +----------------+    +----------------+    +----------------+    +----------------+
 g1 --> | EN             |    |                | g2>| EN             |    |                |
seed -> | in         out | -> | in         out | -> | in         out | -> | in         out | --> result (4)
        +----------------+    +----------------+    +----------------+    +----------------+
```
