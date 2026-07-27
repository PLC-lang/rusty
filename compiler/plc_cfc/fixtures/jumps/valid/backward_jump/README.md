What: a label at a lower priority than the jump that targets it, so control
flows backward — a loop. Exercises a jump landing on an earlier statement.

Illustrated (in priority order):

    LABEL top       (0)
    i --> x         (1)
    cond --> JMP top (2)
