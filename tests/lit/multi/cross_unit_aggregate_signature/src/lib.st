// `make_label` returns an aggregate (STRING) type. AggregateTypeLowerer
// rewrites this POU's signature to take the return as a VAR_IN_OUT
// parameter. main.st calls it from a separate compilation unit, so the
// caller's annotations have to be refreshed against the new signature
// for the call to work.

FUNCTION make_label : STRING[16]
VAR_INPUT
    n : DINT;
END_VAR
    IF n = 0 THEN
        make_label := 'zero';
    ELSE
        make_label := 'item';
    END_IF
END_FUNCTION
