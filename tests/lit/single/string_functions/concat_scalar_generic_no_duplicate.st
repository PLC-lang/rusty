// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// Regression: a nature violation on a generic function with a *scalar* return (here CONCAT_LTOD,
// `<T: ANY_INT> : LTOD`) must be reported exactly once. Such calls are NOT rewritten by the
// aggregate-return lowerer, so their generic nature survives to the regular validation pass. The
// lowerer must therefore stay silent for them; otherwise the diagnostic is emitted twice (once by
// the lowerer, once by validation). The CHECK-NEXT below pins the abort line directly after the
// single E062, so a duplicate E062 would fail the test.

FUNCTION mainProg : LTOD
VAR
    r : REAL;
    i : INT;
END_VAR
    // CHECK: {{.*}}error[E062]: Invalid type nature for generic argument. REAL is no ANY_INT
    // CHECK-NEXT: error: Compilation aborted due to critical errors
    mainProg := CONCAT_LTOD(r, i, i, i);
END_FUNCTION
