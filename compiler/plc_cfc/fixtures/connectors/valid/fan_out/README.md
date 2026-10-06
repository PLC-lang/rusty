What: one connector feeding two continuations — `foo` reaches both sinks.

Illustrated:

    foo --> x>
    >x --> bar (1)
    >x --> baz (2)
