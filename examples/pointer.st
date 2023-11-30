TYPE MyStruct: STRUCT  x: DINT; y: DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
    main := foo();
END_FUNCTION

FUNCTION foo : DINT
VAR
    x : DINT;
    s : MyStruct;
    u,y : REF_TO DINT;
    z : REF_TO REF_TO DINT;
END_VAR

u := NULL;
u := &s.x;
y := u;
z := &y;
s.x := 9;
z^^ := y^*2;
y^ := z^^*2;

foo := y^;
END_FUNCTION
