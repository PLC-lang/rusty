// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that RIGHT with a substring length above the string length clamps
// to the whole string for WSTRING, matching the STRING behavior

FUNCTION main : DINT
VAR
    s : WSTRING;
END_VAR
    s := RIGHT(WSTRING#"sa𝄞ple text", 12);
    // CHECK: length_clamps_to_string
    IF s = "sa𝄞ple text" THEN
        printf('length_clamps_to_string$N');
    END_IF

    s := RIGHT(WSTRING#"sa𝄞ple text", -1);
    // CHECK: negative_length_yields_empty
    IF s = "" THEN
        printf('negative_length_yields_empty$N');
    END_IF

    main := 0;
END_FUNCTION
