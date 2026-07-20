What: a value relayed through a chain of connector/continuation pairs; it
resolves back to the original source as if the hops were a single wire.

Illustrated:

    foo --> a>
    >a --> b>
    >b --> c>
    >c --> bar (0)
