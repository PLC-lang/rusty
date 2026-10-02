What: two connectors share the label `x`, so the named source is ambiguous.

Illustrated:

    foo --> x>
    >x --> bar (1)
    >x --> baz (2)
            x>          (duplicate `x`, rejected)
