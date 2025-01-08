// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb_with_method
VAR_TEMP
    ret : ARRAY[0..1] OF STRING;
    ret_struct : returnStruct;
END_VAR
    METHOD method_with_aggregagte_return: ARRAY[0..1] OF STRING
    VAR_INPUT
        in: STRING;
        in2: STRING;
    END_VAR
        method_with_aggregagte_return[0] := in;
        method_with_aggregagte_return[1] := in2;
    END_METHOD

    METHOD method_with_struct_return: returnStruct
    VAR_INPUT
        in: STRING;
        in2: STRING;
    END_VAR
       method_with_struct_return.a := in;
       method_with_struct_return.b := in2;

       method_with_struct_return.c[0][0] := 'a';
       method_with_struct_return.c[0][1] := 'b';
       method_with_struct_return.c[1][0] := 'c';
       method_with_struct_return.c[1][1] := 'd';
    END_METHOD

    ret := method_with_aggregagte_return('Hello', 'World!');
    printf('%s$N', ADR(ret[0])); // CHECK: Hello
    printf('%s$N', ADR(ret[1])); // CHECK: World!

    ret_struct := method_with_struct_return('Hello', 'Again!');
    printf('%s$N', ADR(ret_struct.a)); // CHECK: Hello
    printf('%s$N', ADR(ret_struct.b)); // CHECK: Again!

    printf('%s$N', ADR(ret_struct.c[0][0])); // CHECK: a
    printf('%s$N', ADR(ret_struct.c[0][1])); // CHECK: b
    printf('%s$N', ADR(ret_struct.c[1][0])); // CHECK: c
    printf('%s$N', ADR(ret_struct.c[1][1])); // CHECK: d
END_FUNCTION_BLOCK

FUNCTION main
VAR
    fb: fb_with_method;
    ret : ARRAY[0..1] OF STRING;
    ret_struct : returnStruct;
END_VAR
    fb();
    ret := fb.method_with_aggregagte_return('Goodbye', 'World!');
    printf('%s$N', ADR(ret[0])); // CHECK: Goodbye
    printf('%s$N', ADR(ret[1])); // CHECK: World!
    ret_struct := fb.method_with_struct_return('Goodbye', 'Again!');
    printf('%s$N', ADR(ret_struct.a)); // CHECK: Goodbye
    printf('%s$N', ADR(ret_struct.b)); // CHECK: Again!
    printf('%s$N', ADR(ret_struct.c[0][0])); // CHECK: a
    printf('%s$N', ADR(ret_struct.c[0][1])); // CHECK: b
    printf('%s$N', ADR(ret_struct.c[1][0])); // CHECK: c
    printf('%s$N', ADR(ret_struct.c[1][1])); // CHECK: d
END_FUNCTION

TYPE returnStruct : STRUCT
    a : STRING;
    b : STRING;
    c : ARRAY[0..1] OF ARRAY[0..1] OF STRING;
END_STRUCT
END_TYPE
