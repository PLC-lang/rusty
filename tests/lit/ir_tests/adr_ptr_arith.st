// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Structural pin for the #1749 fix: `ADR(x) + N` must lower to
// `ptrtoint` + `add` (byte-stride integer arithmetic), not panic. ADR is
// declared `: LWORD` so the binary `+` flows through the integer path; the
// codegen entry now routes pointer operands through
// `convert_to_int_value_if_pointer` (matching the existing comparison-op
// path) instead of an unconditional `into_int_value()` that crashed inkwell.

FUNCTION main
VAR
    arr : ARRAY[0..3] OF BYTE;
    pt  : REF_TO BYTE;
END_VAR

pt := ADR(arr) + 2;
pt^ := BYTE#16#FF;
END_FUNCTION

// CHECK: ptrtoint ptr %{{.*}} to i64
// CHECK-NEXT: add i64 %{{.*}}, 2
// CHECK-NEXT: inttoptr i64 %{{.*}} to ptr
