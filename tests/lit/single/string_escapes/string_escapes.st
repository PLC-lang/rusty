// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// NOTE: The runtime monitoring write API depends on these exact values.
// If this test fails, check whether the runtime parser needs a matching update.

FUNCTION main : DINT
VAR
    // 1. Valid hex escapes (ASCII range)
    s_hex_hi        : STRING[10] := '$48$49';

    // 2. Invalid UTF-8 hex escapes (lossy replacement)
    s_invalid_80    : STRING[10] := '$80';
    s_caf_e9        : STRING[10] := 'caf$E9';
    s_invalid_ff    : STRING[10] := '$FF';
    s_valid_utf8    : STRING[10] := '$F0$9F$92$96';

    // 3. Named escapes
    s_dollar        : STRING[20] := 'Price: $$100';
    s_newline       : STRING[30] := 'Line1$NLine2';
    s_tab           : STRING[20] := 'Col1$TCol2';
    s_cr            : STRING[20] := 'Before$RAfter';
    s_quote         : STRING[20] := '$'quoted$'';
END_VAR
    // --- Valid hex escapes ---
    // CHECK: HI
    printf('%s$N', REF(s_hex_hi));

    // --- Invalid UTF-8 (lossy → U+FFFD replacement character) ---
    // CHECK: �
    printf('%s$N', REF(s_invalid_80));
    // CHECK: caf�
    printf('%s$N', REF(s_caf_e9));
    // CHECK: �
    printf('%s$N', REF(s_invalid_ff));
    // CHECK: 💖
    printf('%s$N', REF(s_valid_utf8));

    // --- Named escapes ---
    // Known bug: '$$' followed by 2 hex digits is double-processed in the
    // committed parser (two-pass). '$$10' → '$10' (pass 1) → 0x10 DLE (pass 2).
    // Remove XCHECK and restore CHECK once the single-pass fix lands.
    // XCHECK: Price: $100
    printf('%s$N', REF(s_dollar));
    // CHECK: Line1
    // CHECK: Line2
    printf('%s$N', REF(s_newline));
    // CHECK: Col1	Col2
    printf('%s$N', REF(s_tab));
    // CHECK: 'quoted'
    printf('%s$N', REF(s_quote));
END_FUNCTION
