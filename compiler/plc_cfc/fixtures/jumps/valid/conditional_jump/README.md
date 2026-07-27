What: a jump that skips an assignment when its wired condition is true.

Illustrated:

    myCondition --> JMP skipAssignment (0)

    x --> y (1)

    LABEL skipAssignment (2)
