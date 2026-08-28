Pins the deliberate permissiveness of storage modes on numeric sinks
(PRG-4559 deviates from its acceptance criteria here): the ST compiler
accepts `<dint> := TRUE`, so a `DINT` sink with Set/Reset engraves the
literal `1`/`0` instead of raising a validation error. One source fans out
to a Set and a Reset sink; a FALSE input leaves both preset values alone.
