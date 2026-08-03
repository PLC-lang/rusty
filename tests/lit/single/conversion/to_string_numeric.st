// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for PRG-4494: the generic TO_STRING / TO_WSTRING must bind to the concrete per-type
// monomorphization (e.g. TO_STRING__DINT) instead of silently falling back to TO_STRING__STRING
// and reinterpreting the argument's bytes. DINT#-1 is a value that used to core-dump.
FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := TO_STRING(DINT#12345);          printf('%s$N', REF(s));   // CHECK: 12345
    s := TO_STRING(DINT#-1);             printf('%s$N', REF(s));   // CHECK: -1
    s := TO_STRING(LINT#-9);             printf('%s$N', REF(s));   // CHECK: -9
    s := TO_STRING(USINT#200);           printf('%s$N', REF(s));   // CHECK: 200
    s := TO_STRING(BYTE#16#FF);          printf('%s$N', REF(s));   // CHECK: 255
    s := TO_STRING(REAL#3.14);           printf('%s$N', REF(s));   // CHECK: 3.140000
    s := TO_STRING(TIME#1s500ms);        printf('%s$N', REF(s));   // CHECK: 1s500ms

    // TO_WSTRING binds to its own per-type monomorph; round-trip back through TO_STRING to print it.
    s := TO_STRING(TO_WSTRING(DINT#42)); printf('%s$N', REF(s));   // CHECK: 42
END_FUNCTION
