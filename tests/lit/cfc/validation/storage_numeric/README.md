Pins that storage modes (Set/Reset) on a numeric sink are rejected: the
generated `IF a THEN ds := TRUE; END_IF` stores a BOOL into a `DINT`, which
the assignment validation reports as E037 at each sink block. One source
fans out to a Set sink and a Reset sink, so the build fails with two
diagnostics.
