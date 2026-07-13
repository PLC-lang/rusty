What: a connector/continuation pair that nothing reads. It produces no
statement and, being unconsumed, raises no diagnostic even though the connector
has no input.

Illustrated:

    x>       (no source)
    >x       (nobody reads it)
