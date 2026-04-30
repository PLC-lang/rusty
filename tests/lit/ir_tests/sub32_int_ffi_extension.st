// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Regression for #1146. Verify call sites and declarations of functions taking sub-32-bit
// integer parameters carry the correct LLVM `signext`/`zeroext` parameter attributes.
// Without these, optimized release builds of FFI'd callees (e.g. the Rust stdlib) read
// garbage upper register bits instead of the intended signed value — see also
// `cargo test --release -p iec61131std mul_ltime_signed`.

{external}
FUNCTION extFunc : LINT
VAR_INPUT
    in1 : SINT;
    in2 : USINT;
    in3 : INT;
    in4 : UINT;
    in5 : BOOL;
END_VAR
END_FUNCTION

PROGRAM main
VAR
    a : LINT;
END_VAR
    a := extFunc(SINT#-1, USINT#1, INT#-1, UINT#1, FALSE);
END_PROGRAM

// BOOL is lowered to `i8` at the FFI boundary, not `i1`; both still get `zeroext`.
// `CHECK-DAG` so the order of `call`/`declare` (call first under --debug, declare last)
// doesn't matter to this test.
// CHECK-DAG: call i64 @extFunc(i8 signext {{.*}}, i8 zeroext {{.*}}, i16 signext {{.*}}, i16 zeroext {{.*}}, i8 zeroext {{.*}})
// CHECK-DAG: declare i64 @extFunc(i8 signext, i8 zeroext, i16 signext, i16 zeroext, i8 zeroext)
