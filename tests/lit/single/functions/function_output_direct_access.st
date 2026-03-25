// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that FUNCTION VAR_OUTPUT with direct-access targets (e.g. wordVar.%B0)
// correctly writes the output value back into the target variable using
// proper bit-manipulation (mask/shift/or).

FUNCTION FLIP
    VAR_INPUT
        in: BYTE;
    END_VAR
    VAR_OUTPUT
        out : BYTE;
    END_VAR
    out := NOT(in);
END_FUNCTION

FUNCTION SET_BIT
    VAR_INPUT
        val : BOOL;
    END_VAR
    VAR_OUTPUT
        result : BOOL;
    END_VAR
    result := val;
END_FUNCTION

FUNCTION main : DINT
VAR
    wordVar : WORD;
    dwordVar : DWORD;
    byteVar : BYTE;
END_VAR
    // Implicit call, byte-access on low byte
    wordVar := 0;
    FLIP(2#01010101, wordVar.%B0);
    printf('implicit_B0=%d$N', wordVar);  // CHECK: implicit_B0=170

    // Explicit output assignment, byte-access on low byte
    wordVar := 0;
    FLIP(in := 2#01010101, out => wordVar.%B0);
    printf('explicit_B0=%d$N', wordVar);  // CHECK: explicit_B0=170

    // Byte-access on high byte
    wordVar := 0;
    FLIP(2#01010101, wordVar.%B1);
    printf('B1=%d$N', wordVar);           // CHECK: B1=43520

    // Bit-access via function output
    byteVar := 0;
    SET_BIT(TRUE, byteVar.%X3);
    printf('bit3=%d$N', byteVar);         // CHECK: bit3=8

    // DWORD byte-access at offset 2
    dwordVar := 0;
    FLIP(16#55, dwordVar.%B2);
    printf('dword_B2=%d$N', dwordVar);    // CHECK: dword_B2=11141120

    // Verify existing bits are preserved
    wordVar := 16#FF00;
    FLIP(2#01010101, wordVar.%B0);
    printf('preserve=%d$N', wordVar);     // CHECK: preserve=65450

    main := 0;
END_FUNCTION
