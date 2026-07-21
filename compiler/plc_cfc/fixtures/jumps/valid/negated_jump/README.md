What: a jump whose negation bubble inverts its wired condition (fires when the
condition is false).

Illustrated:

    myCondition --o JMP skipAssignment (0)

    x --> y (1)

    LABEL skipAssignment (2)
