What: a jump that skips an assignment when its wired condition is true.

Illustrated:

    myCondition --> JMP skipAssignment (1)

    x --> y (2)

    LABEL skipAssignment (3)
