What: one connector feeding two continuations — `foo` reaches both sinks.

Illustrated:

    foo --> x>
    >x --> bar (0)
    >x --> baz (1)
