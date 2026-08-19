Runtime proof of the `Set` storage mode: `a --> [b |S]` lowers to
`IF a THEN b := TRUE; END_IF`. A FALSE input writes nothing, a TRUE input
latches the sink, and the latch holds after the input drops again.
