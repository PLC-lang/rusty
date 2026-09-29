// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
//
// AND, OR and XOR take two or more arguments, MOD takes exactly two, NOT takes exactly one.

FUNCTION main
VAR
    a : BOOL;
    x : DINT;
END_VAR

// CHECK: {{.*}}error[E032]{{.*}}this POU takes 2 arguments but 1 argument was supplied{{.*}}
a := AND(a);

// CHECK: {{.*}}error[E032]{{.*}}this POU takes 2 arguments but 1 argument was supplied{{.*}}
a := OR(a);

// CHECK: {{.*}}error[E032]{{.*}}this POU takes 2 arguments but 1 argument was supplied{{.*}}
a := XOR(a);

// CHECK: {{.*}}error[E032]{{.*}}this POU takes 2 arguments but 3 arguments were supplied{{.*}}
x := MOD(x, x, x);

// CHECK: {{.*}}error[E032]{{.*}}this POU takes 1 argument but 2 arguments were supplied{{.*}}
a := NOT(a, a);
END_FUNCTION
