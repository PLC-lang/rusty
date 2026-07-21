What: two labels sharing the name `dup`, making a jump to it ambiguous (E144).

Illustrated:

    myCondition --> JMP dup (0)

    LABEL dup (1)
    LABEL dup (2)
