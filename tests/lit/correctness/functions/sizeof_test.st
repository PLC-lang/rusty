// RUN: (%COMPILE %s && %RUN) | %CHECK %s

CLASS MyClass
    VAR
    x, y: INT; // 4 bytes
    END_VAR
END_CLASS

TYPE MyStruct: STRUCT
    a: BYTE; // 8bit - offset 0 -> 1 byte
    b: DWORD; // 32bit - offset 32 -> 8 bytes
    c: WORD; // 16bit - offset 64 -> 10 bytes
    d: LWORD; // 64bit - offset 128 -> 24 bytes
END_STRUCT END_TYPE

FUNCTION main
    VAR
        s1: SINT;
        s2: INT;
        s3: DINT;
        s4: LINT;
        s5: USINT;
        s6: UDINT;
        s7: ULINT;
        s8: LINT;
    END_VAR

    VAR_TEMP
        t1: MyStruct;
        t2: STRING;
        t3: WCHAR;
        t4: MyClass;
        t5: LREAL;
        t6: BOOL;
    END_VAR

    s1 := SIZEOF(t6);
    s2 := SIZEOF(s2);
    s3 := SIZEOF(t5);
    s4 := SIZEOF(t1);
    s5 := SIZEOF(REF(s1));
    s6 := SIZEOF(t2);
    s7 := SIZEOF(t3);
    s8 := SIZEOF(t4);

    printf('s1 = %d$N', s1); // CHECK: s1 = 1
    printf('s2 = %d$N', s2); // CHECK: s2 = 2
    printf('s3 = %d$N', s3); // CHECK: s3 = 8
    printf('s4 = %d$N', s4); // CHECK: s4 = 24
    printf('s5 = %d$N', s5); // CHECK: s5 = 8
    printf('s6 = %d$N', s6); // CHECK: s6 = 81
    printf('s7 = %d$N', s7); // CHECK: s7 = 2
    printf('s8 = %d$N', s8); // CHECK: s8 = 16
END_FUNCTION