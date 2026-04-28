// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    g : DINT := 21;
END_VAR

TYPE MyStruct : STRUCT
    r : REFERENCE TO DINT;
END_STRUCT
END_TYPE

VAR_GLOBAL
    s : MyStruct := (r := (REF(g)));
END_VAR

PROGRAM prog
    // CHECK: 21
    printf('%d$N', s.r);
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
