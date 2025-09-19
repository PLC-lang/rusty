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

    mainProg(valueLREAL_neg, valueREAL_neg, funcLREAL_huge, funcREAL_tiny);
END_FUNCTION

PROGRAM mainProg
VAR
    // Basic positive/negative values
    valueLREAL : LREAL := 3.14;
    valueREAL : REAL := 3.14;
    valueLREAL_neg : LREAL := -3.14159;
    valueREAL_neg : REAL := -3.14159;

    // Zero values
    valueLREAL_zero : LREAL := 0.0;
    valueREAL_zero : REAL := 0.0;
    valueLREAL_neg_zero : LREAL := -0.0;

    // Integer literals assigned to float types (type promotion)
    valueLREAL_int : LREAL := 42;
    valueREAL_int : REAL := -42;

    // Very large integers that require double precision
    valueLREAL_big_int : LREAL := 123456789012345;
    valueREAL_big_int : REAL := 123456789012345;

    // Scientific notation
    valueLREAL_sci : LREAL := 1.25e-38;
    valueREAL_sci : REAL := 1.25e6;
    valueLREAL_sci_neg : LREAL := -1.25e-38;

    // Numbers that overflow f32 but fit in f64
    valueLREAL_f32_overflow : LREAL := 1e100;

    // High precision decimals
    valueLREAL_precision : LREAL := 3.141592653589793238462643383279;
    valueREAL_precision : REAL := 3.141592653589793238462643383279;
END_VAR
VAR_IN_OUT
    // Reference parameters with edge cases
    ioLREAL_neg : LREAL := -3.14159;
    ioREAL_neg : REAL := -3.14159;
    ioLREAL_huge : LREAL := 1e100;
    ioREAL_tiny : REAL := 1e-20;
END_VAR
VAR_TEMP
    // Temporary variables with various values
    tmpLREAL_neg : LREAL := -6.666;
    tmpREAL_neg : REAL := -3.33;
    tmpLREAL_exp : LREAL := 2.718281828459045;
    tmpREAL_exp : REAL := 2.718281828459045;
END_VAR
    // CHECK: Value LREAL: 3.140000
    printf('Value LREAL: %f$N', valueLREAL);
    // CHECK: Value REAL: 3.140000
    printf('Value REAL: %f$N', valueREAL);
    // CHECK: Value LREAL negative: -3.141590
    printf('Value LREAL negative: %f$N', valueLREAL_neg);
    // CHECK: Value REAL negative: -3.141590
    printf('Value REAL negative: %f$N', valueREAL_neg);

    // Zero Values
    // CHECK: LREAL zero: 0.000000
    printf('LREAL zero: %f$N', valueLREAL_zero);
    // CHECK: REAL zero: 0.000000
    printf('REAL zero: %f$N', valueREAL_zero);
    // CHECK: LREAL negative zero: -0.000000
    printf('LREAL negative zero: %f$N', valueLREAL_neg_zero);

    // Integer Promotion
    // CHECK: LREAL from int: 42.000000
    printf('LREAL from int: %f$N', valueLREAL_int);
    // CHECK: REAL from int: -42.000000
    printf('REAL from int: %f$N', valueREAL_int);
    // CHECK: LREAL big int: 123456789012345.000000
    printf('LREAL big int: %f$N', valueLREAL_big_int);
    // CHECK: REAL big int: 123456788103168.000000
    printf('REAL big int: %f$N', valueREAL_big_int);

    // Scientific Notation
    // CHECK: LREAL scientific: 1.250000e-38
    printf('LREAL scientific: %e$N', valueLREAL_sci);
    // CHECK: REAL scientific: 1250000.000000
    printf('REAL scientific: %f$N', valueREAL_sci);
    // CHECK: LREAL sci negative: -1.250000e-38
    printf('LREAL sci negative: %e$N', valueLREAL_sci_neg);

    // Edge Cases
    // CHECK: LREAL overflow f32: 1.000000e+100
    printf('LREAL overflow f32: %e$N', valueLREAL_f32_overflow);
    // CHECK: LREAL precision: 3.141592653589793
    printf('LREAL precision: %.15f$N', valueLREAL_precision);
    // CHECK: REAL precision: 3.141593
    printf('REAL precision: %.6f$N', valueREAL_precision);

    // Reference Parameters
    // CHECK: inout LREAL negative: -3.141590
    printf('inout LREAL negative: %f$N', ioLREAL_neg);
    // CHECK: inout REAL negative: -3.141590
    printf('inout REAL negative: %f$N', ioREAL_neg);
    // CHECK: inout LREAL huge: 1.000000e+100
    printf('inout LREAL huge: %e$N', ioLREAL_huge);
    // CHECK: inout REAL tiny: 1.000000e-20
    printf('inout REAL tiny: %e$N', ioREAL_tiny);

    // Temporary Variables
    // CHECK: stack LREAL negative: -6.666000
    printf('stack LREAL negative: %f$N', tmpLREAL_neg);
    // CHECK: stack REAL negative: -3.330000
    printf('stack REAL negative: %f$N', tmpREAL_neg);
    // CHECK: stack LREAL exp: 2.718282
    printf('stack LREAL exp: %f$N', tmpLREAL_exp);
    // CHECK: stack REAL exp: 2.718282
    printf('stack REAL exp: %f$N', tmpREAL_exp);

    // Runtime Assignment Test
    // CHECK: Before assignment: -3.141590
    printf('Before assignment: %f$N', valueLREAL_neg);
    valueLREAL_neg := -1.111111111;
    // CHECK: After assignment: -1.111111
    printf('After assignment: %f$N', valueLREAL_neg);

    // Test very large number assignment
    valueLREAL_f32_overflow := 2e100;
    // CHECK: Large number assignment: 2.000000e+100
    printf('Large number assignment: %e$N', valueLREAL_f32_overflow);
END_PROGRAM
