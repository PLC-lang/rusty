// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE MyStruct : STRUCT
    field : DINT;
END_STRUCT
END_TYPE

VAR_GLOBAL
    local : DINT := 5;
END_VAR

FUNCTION_BLOCK Foo
VAR
    local : DINT := 42;
    s : MyStruct := (field := local);
END_VAR
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    fb : Foo;
END_VAR
    // CHECK: 42
    printf('%d$N', fb.s.field);
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
