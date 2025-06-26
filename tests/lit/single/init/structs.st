// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL CONSTANT
    constantDintValue : DINT := 12345;
END_VAR
TYPE myStruct : STRUCT
        member : REF_TO STRING := REF(s);
        member2 AT s2 : ARRAY[0..1] OF STRING;
        member3: DINT := 367;
        member4: DINT := 12345;
    END_STRUCT
END_TYPE

VAR_GLOBAL
    s : STRING := 'Hello, world!';
    s2 : ARRAY[0..1] OF STRING := ['hello', 'world'];
END_VAR

PROGRAM prog
VAR
    str: myStruct;
END_VAR
VAR_TEMP
    tmp: STRING;
END_VAR
    // CHECK: Hello, world!
    printf('%s$N', str.member);
    // CHECK: hello world
    tmp := str.member2[0];
    printf('%s', REF(tmp));
    tmp := str.member2[1];
    printf(' %s$N', REF(tmp));
    // CHECK: 367
    printf('%d$N', str.member3);
    // CHECK: 12345
    printf('%d$N', str.member4);
END_PROGRAM

FUNCTION main: DINT
    prog();
END_FUNCTION
