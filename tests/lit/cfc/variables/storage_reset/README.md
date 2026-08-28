Runtime proof of the `Reset` storage mode: `a --> [b |R]` lowers to
`IF a THEN b := FALSE; END_IF`. A FALSE input leaves a preset TRUE sink
untouched, a TRUE input clears it, and it stays cleared after the input drops.
