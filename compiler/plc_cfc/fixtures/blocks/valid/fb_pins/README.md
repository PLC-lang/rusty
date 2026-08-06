What: negation bubbles on the pins of a stateful block (a function-block
instance): the input pin inverts the passed value, the output pin inverts the
member read. Verbatim IDE export.

Illustrated:
```
           +---- inst : counter ----+
a1 ------o-| in                 out |-o------> b1 (1)
           +------------------------+ (0)
```
