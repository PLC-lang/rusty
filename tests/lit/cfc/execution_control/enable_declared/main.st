FUNCTION main : DINT
    // EN and ENO are not reserved words: the caller declares ordinary BOOL
    // variables with these names and wires them into the extension pins.
    enable_declared();
    // CHECK: idle: out = 0, eno = FALSE
    printf('idle: out = %d, eno = %s$N', enable_declared.localOut, SEL(enable_declared.ENO, 'FALSE', 'TRUE'));

    enable_declared.EN := TRUE;
    enable_declared();
    // CHECK: run: out = 5, eno = TRUE
    printf('run: out = %d, eno = %s$N', enable_declared.localOut, SEL(enable_declared.ENO, 'FALSE', 'TRUE'));

    // EN back to FALSE: the declared variable really gates; the counter holds
    // its 5, a broken gate would accumulate to 10.
    enable_declared.EN := FALSE;
    enable_declared();
    // CHECK: hold: out = 5, eno = FALSE
    printf('hold: out = %d, eno = %s$N', enable_declared.localOut, SEL(enable_declared.ENO, 'FALSE', 'TRUE'));
END_FUNCTION
