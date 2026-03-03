// CHECK: 42
// CHECK: 10

TYPE MyStruct : STRUCT
    fb : FbA;
    v  : myINT;
END_STRUCT
END_TYPE

FUNCTION main : DINT
VAR
    s : MyStruct;
END_VAR
    printf('%d$N', s.fb.x);
    printf('%d$N', s.v);
END_FUNCTION
