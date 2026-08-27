End-to-end runtime check of the `Reference` storage mode: the sink transpiles
to `b REF= a`, so after one cycle `b` points at `a` and reads through to
whatever value `a` currently holds.
