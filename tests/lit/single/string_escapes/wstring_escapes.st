// RUN: %COMPILE %s && %RUN | %CHECK %s

// NOTE: The runtime monitoring write API depends on these exact values.
// If this test fails, check whether the runtime parser needs a matching update.

FUNCTION main : DINT
VAR
    // 4. WSTRING — valid hex escapes
    w_hex_hi        : WSTRING[10] := "$0048$0049";
    w_surrogate     : WSTRING[10] := "$D83D$DE00";

    // 5. WSTRING — unpaired surrogate replacement
    w_high_only     : WSTRING[10] := "$D800";
    w_low_only      : WSTRING[10] := "$DE00";
    w_high_not_low  : WSTRING[10] := "$D83D$0041";
    w_reversed      : WSTRING[10] := "$DE00$D83D";

    // 6. WSTRING — named escapes
    w_dollar        : WSTRING[20] := "Price: $$100";
    w_newline       : WSTRING[30] := "Line1$NLine2";
    w_dquote        : WSTRING[20] := "$"quoted$"";
END_VAR
    // --- Valid hex escapes ---
    // CHECK: HI
    printf('%s$N', REF(TO_STRING(w_hex_hi)));
    // CHECK: 😀
    printf('%s$N', REF(TO_STRING(w_surrogate)));

    // --- Unpaired surrogates (lossy → U+FFFD) ---
    // CHECK: �
    printf('%s$N', REF(TO_STRING(w_high_only)));
    // CHECK: �
    printf('%s$N', REF(TO_STRING(w_low_only)));
    // CHECK: �A
    printf('%s$N', REF(TO_STRING(w_high_not_low)));
    // CHECK: ��
    printf('%s$N', REF(TO_STRING(w_reversed)));

    // --- Named escapes ---
    // CHECK: Price: $100
    printf('%s$N', REF(TO_STRING(w_dollar)));
    // CHECK: Line1
    // CHECK: Line2
    printf('%s$N', REF(TO_STRING(w_newline)));
    // CHECK: "quoted"
    printf('%s$N', REF(TO_STRING(w_dquote)));
END_FUNCTION
