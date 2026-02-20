// Issue #1522: Typedef initializer chains not propagating values
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE myStruct: STRUCT
  a : INT;
END_STRUCT
END_TYPE

TYPE mytypedefstruct: myStruct := (a := 3); END_TYPE
TYPE mysubtype: mytypedefstruct; END_TYPE
TYPE mysubtype2: mysubtype; END_TYPE

FUNCTION main : DINT
VAR
    inst: mysubtype2;
END_VAR
    printf('a=%d$N', inst.a); // CHECK: a=3
    main := 0;
END_FUNCTION
