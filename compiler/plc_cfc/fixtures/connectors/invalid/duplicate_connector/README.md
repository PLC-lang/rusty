What: two connectors share the label `x`, so the named source is ambiguous.

Illustrated:

    foo --> x>
    >x --> bar (0)
    >x --> baz (1)
            x>          (duplicate `x`, rejected)
