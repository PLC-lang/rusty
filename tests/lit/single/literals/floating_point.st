// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    // Basic global variables
    g_LREAL_pos : LREAL := 12.1212;
    g_LREAL_neg : LREAL := -12.1212;
    g_REAL_pos : REAL := 15.62;
    g_REAL_neg : REAL := -15.62;

    // Edge cases - very large numbers (should trigger LREAL inference)
    g_LREAL_huge : LREAL := 1e100;
    g_LREAL_huge_neg : LREAL := -1e100;

    // Edge cases - very small numbers (close to zero)
    g_LREAL_tiny : LREAL := 1e-100;
    g_LREAL_tiny_neg : LREAL := -1e-100;

    // High precision numbers (should preserve precision in LREAL)
    g_LREAL_precise : LREAL := 1.23456789012345678901234567890;
    g_REAL_precise : REAL := 1.23456789012345678901234567890;

    // Numbers at f32 limits
    g_LREAL_f32_max : LREAL := 3.4028235e38;
    g_LREAL_over_f32 : LREAL := 3.5e38;
END_VAR

FUNCTION main
VAR
    // Function-local variables with edge cases
    valueLREAL : LREAL := 3.14;
    valueREAL : REAL := 3.14;
    valueLREAL_neg : LREAL := -3.14159;
    valueREAL_neg : REAL := -3.14159;
    funcLREAL_huge : LREAL := 1e100;
    funcREAL_precise : REAL := 1.234567890123456789;
    funcLREAL_tiny : LREAL := 1e-200;
    funcREAL_tiny : REAL := 1e-20;
END_VAR
    // Function Local Variables
    // CHECK: Function LREAL: 3.140000
    printf('Function LREAL: %f$N', valueLREAL);
    // CHECK: Function REAL: 3.140000
    printf('Function REAL: %f$N', valueREAL);
    // CHECK: Function LREAL negative: -3.141590
    printf('Function LREAL negative: %f$N', valueLREAL_neg);
    // CHECK: Function REAL negative: -3.141590
    printf('Function REAL negative: %f$N', valueREAL_neg);

    // Function Edge Cases
    // CHECK: Function LREAL huge: 1.000000e+100
    printf('Function LREAL huge: %e$N', funcLREAL_huge);
    // CHECK: Function REAL precise: 1.234568
    printf('Function REAL precise: %f$N', funcREAL_precise);
    // CHECK: Function LREAL tiny: 1.000000e-200
    printf('Function LREAL tiny: %e$N', funcLREAL_tiny);

    // Global Variables
    // CHECK: Global LREAL positive: 12.121200
    printf('Global LREAL positive: %f$N', g_LREAL_pos);
    // CHECK: Global LREAL negative: -12.121200
    printf('Global LREAL negative: %f$N', g_LREAL_neg);
    // CHECK: Global REAL positive: 15.620000
    printf('Global REAL positive: %f$N', g_REAL_pos);
    // CHECK: Global REAL negative: -15.620000
    printf('Global REAL negative: %f$N', g_REAL_neg);

    // Global Edge Cases
    // CHECK: Global LREAL huge: 1.000000e+100
    printf('Global LREAL huge: %e$N', g_LREAL_huge);
    // CHECK: Global LREAL huge neg: -1.000000e+100
    printf('Global LREAL huge neg: %e$N', g_LREAL_huge_neg);
    // CHECK: Global LREAL tiny: 1.000000e-100
    printf('Global LREAL tiny: %e$N', g_LREAL_tiny);
    // CHECK: Global LREAL tiny neg: -1.000000e-100
    printf('Global LREAL tiny neg: %e$N', g_LREAL_tiny_neg);
    // CHECK: Global LREAL precise: 1.234567890123457
    printf('Global LREAL precise: %.15f$N', g_LREAL_precise);
    // CHECK: Global REAL precise: 1.234568
    printf('Global REAL precise: %f$N', g_REAL_precise);
    // CHECK: Global LREAL f32 max: 3.402823e+38
    printf('Global LREAL f32 max: %e$N', g_LREAL_f32_max);
    // CHECK: Global LREAL over f32: 3.500000e+38
    printf('Global LREAL over f32: %e$N', g_LREAL_over_f32);
END_FUNCTION
