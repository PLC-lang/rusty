What: a network whose document order is shuffled against evaluation priority,
with three jumps (two fanning into one label `end`) and two labels. Exercises
that statements — jumps, labels, and sinks alike — are ordered purely by
`EvaluationPriority`.

Illustrated (in priority order):

    g1 --> JMP mid  (0)
    x  --> a        (1)
    g2 --> JMP end  (2)
    LABEL mid       (3)
    x  --> b        (4)
    g3 --> JMP end  (5)
    x  --> c        (6)
    LABEL end       (7)
