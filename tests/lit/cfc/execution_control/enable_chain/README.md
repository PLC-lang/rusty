What: a four-block chain that alternates gated and ungated blocks, with the two
gates toggled in sync and out of sync. Ungated blocks run every cycle and
consume whatever their producer holds, so a disabled head feeds its buffered
value downstream, and a disabled tail freezes while the head keeps advancing.

Illustrated:
```
          c : counter (1)      d1 : doubler (2)     p : addOne (3)       d2 : doubler (4)
        +----------------+    +----------------+    +----------------+    +----------------+
 g1 --> | EN             |    |                | g2>| EN             |    |                |
seed -> | in         out | -> | in         out | -> | in         out | -> | in         out | --> result (5)
        +----------------+    +----------------+    +----------------+    +----------------+
```
