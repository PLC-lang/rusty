What: `bar := foo` routed through a connector/continuation pair (`x`) instead of
a direct wire.

Illustrated:

    foo --> x>
    >x --> bar (0)
