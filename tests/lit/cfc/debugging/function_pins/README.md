What: the debug info a diagram produces. The declaration's members keep text
lines in `function_pins.cfc`; the block call, the two sinks and the captured
block output live in the diagram's own debug file `function_pins.cfc.function_pins`,
scoped by a lexical block, with their execution order plus one as the line
(DWARF reserves line 0).

Illustrated:
```
           +-------- myAdd ---------+
a1 ------o-| IN1              myAdd |-o------> b1 (2)
a2 --------| IN2       myAddDoubled |-o------> b2 (1)
           +------------------------+ (0)
```
