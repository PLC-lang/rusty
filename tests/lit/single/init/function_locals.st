// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    ps: STRING := 'Hello world!';
    ps2: STRING;
    x : DINT := 1337;
END_VAR

FUNCTION_BLOCK foo
VAR
    s AT ps: STRING;
END_VAR
VAR_TEMP
    s2 AT ps2: STRING;
END_VAR
    s2 := 'Hello from me, too';

    // CHECK: Hello world!
    printf('%s$N', REF(s));
    // CHECK: Hello from me, too
    printf('%s$N', REF(s2));
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar
VAR
    x : DINT := 69;
    shadowed_global : REF_TO DINT := REF(x);
    alias_to_local AT x : DINT;
    reference_to_local : REFERENCE TO DINT REF= x;
END_VAR
VAR_TEMP
    y : DINT := 420;
    temp_ref_to_temp : REF_TO DINT := REF(y);
    temp_alias AT y :DINT;
    temp_reference_to : REFERENCE TO DINT REF= y;
END_VAR
    printf('%d$N', shadowed_global^); // CHECK 69
    printf('%d$N', temp_ref_to_temp^); // CHECK 420
    printf('%d$N', alias_to_local); // CHECK 69
    printf('%d$N', reference_to_local); // CHECK 69
    printf('%d$N', temp_alias); // CHECK 420
    printf('%d$N', temp_reference_to); // CHECK 420
END_FUNCTION_BLOCK

FUNCTION main: DINT
VAR
    fb: foo;
    bb: bar;

    z : DINT := 42;
    temp_ref_to_temp : REF_TO DINT := REF(z);
    temp_alias AT z :DINT;
    temp_reference_to : REFERENCE TO DINT REF= z;
END_VAR
    fb();
    bb();
    baz();

    printf('%d$N', temp_ref_to_temp^); // CHECK 42
    printf('%d$N', temp_alias); // CHECK 42
    printf('%d$N', temp_reference_to); // CHECK 42
END_FUNCTION

FUNCTION baz
    VAR
        s1: REF_TO LINT := REF(t1);    
        s2 AT t1 : LINT;               
        s3 : REFERENCE TO LINT REF= t1;
        s4 AT s2 : LINT;               
        s5 : REF_TO LINT := REF(s4);   
        s6 : REFERENCE TO LINT REF= s4;
    END_VAR
    VAR_TEMP
        t1 : LINT := 16#DEADBEEF;
        t2 : REF_TO LINT := REF(t1);   
        t3 AT s2 : LINT;               
        t4 : REFERENCE TO LINT REF= s4;
    END_VAR
        printf('%#010x$N', s1^);    // CHECK 0xdeadbeef
        printf('%#010x$N', s2);     // CHECK 0xdeadbeef
        printf('%#010x$N', s3);     // CHECK 0xdeadbeef
        printf('%#010x$N', s4);     // CHECK 0xdeadbeef
        printf('%#010x$N', s5^);    // CHECK 0xdeadbeef
        printf('%#010x$N', s6);     // CHECK 0xdeadbeef
        printf('%#010x$N', t2^);    // CHECK 0xdeadbeef
        printf('%#010x$N', t3);     // CHECK 0xdeadbeef
        printf('%#010x$N', t4);     // CHECK 0xdeadbeef
END_FUNCTION