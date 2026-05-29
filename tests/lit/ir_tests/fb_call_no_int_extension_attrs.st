// REQUIRES: host-x86_64
// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Direct FUNCTION_BLOCK invocations (e.g. `myFbInstance(in_val := …)`) lower to
// a gep+store of each argument into the instance struct followed by a call to
// the FB body that takes only the instance pointer — there are no scalar
// arguments in the LLVM call signature, so no `signext`/`zeroext` attribute is
// applicable. Regression for #1146 review feedback: confirms the call-side
// integer-extension pass leaves direct FB calls alone.
//
// Gated to host-x86_64 to mirror sub32_int_ffi_extension.st: `signext`/`zeroext`
// are only emitted on x86 family and Apple/Darwin targets. The point of this
// test is the *absence* of those tokens, but on AAPCS targets they would be
// absent for a different reason (target gate), which would make the test
// trivially green and hide a future regression on x86.

FUNCTION_BLOCK myFb
VAR_INPUT
    in_val : SINT;
END_VAR
END_FUNCTION_BLOCK

PROGRAM main
VAR
    instance : myFb;
END_VAR
    instance(in_val := SINT#-1);
END_PROGRAM

// The body call passes only the instance struct pointer (LLVM 21 opaque
// `ptr`) — no `signext`/`zeroext` token should appear on the call line.
// CHECK: call void @myFb(ptr {{.*}})
// CHECK-NOT: call void @myFb({{.*}}signext{{.*}})
// CHECK-NOT: call void @myFb({{.*}}zeroext{{.*}})
