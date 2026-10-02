What: the debug info a diagram produces. The declaration's members keep text
lines in `function_pins.cfc`; the block call, the two sinks and the captured
block output live in the diagram's own debug file `function_pins.cfc.function_pins`,
scoped by a lexical block, with their execution order as the line. The last
side effect of each element is its key instruction, the only place a debugger
stops.

Illustrated:
```
           +-------- myAdd ---------+
a1 ------o-| IN1              myAdd |-o------> b1 (3)
a2 --------| IN2       myAddDoubled |-o------> b2 (2)
           +------------------------+ (1)
```
