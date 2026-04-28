// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE MyStruct : STRUCT
    fb : FbA := (x := 10);
END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
VAR
    x : DINT;
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    s : MyStruct;
END_VAR
    // CHECK: s.fb.x = 10
    printf('s.fb.x = %d$N', s.fb.x);
END_FUNCTION
