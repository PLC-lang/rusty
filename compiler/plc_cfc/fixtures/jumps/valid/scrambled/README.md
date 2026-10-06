What: a network whose document order is shuffled against evaluation priority,
with three jumps (two fanning into one label `end`) and two labels. Exercises
that statements — jumps, labels, and sinks alike — are ordered purely by
`EvaluationPriority`.

Illustrated (in priority order):

    g1 --> JMP mid  (1)
    x  --> a        (2)
    g2 --> JMP end  (3)
    LABEL mid       (4)
    x  --> b        (5)
    g3 --> JMP end  (6)
    x  --> c        (7)
    LABEL end       (8)
