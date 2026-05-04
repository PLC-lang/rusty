// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for #1463: `methodCall()^ := value` must parse and write through
// the returned pointer. Prior to the fix, the parser closed the call and
// rejected `^` with E007.

FUNCTION_BLOCK Holder
VAR
    x : DINT := 0;
END_VAR
METHOD GetPtr : REF_TO DINT
    GetPtr := REF(x);
END_METHOD
METHOD Read : DINT
    Read := x;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    h : Holder;
END_VAR
    h.GetPtr()^ := 17;
    // CHECK: 17
    printf('%d$N', h.Read());
    main := 0;
END_FUNCTION
