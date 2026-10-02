What: a jump whose negation bubble inverts its wired condition (fires when the
condition is false).

Illustrated:

    myCondition --o JMP skipAssignment (1)

    x --> y (2)

    LABEL skipAssignment (3)
