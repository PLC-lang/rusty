What: a jump with no wired condition. It lowers to a `FALSE` guard, so it can
never be taken (a warning, E145); the assignment always runs.

Illustrated:

    (unwired) --> JMP skipAssignment (0)

    x --> y (1)

    LABEL skipAssignment (2)
