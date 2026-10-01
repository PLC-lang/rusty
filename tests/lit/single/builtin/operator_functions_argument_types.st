// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
//
// MOD takes numbers, AND, OR and XOR take bit and integer values. Every other argument is
// rejected with E062, the same for the implicit and the explicit (named) call form. In a call that
// mixes types, only the argument of the wrong type is reported.

FUNCTION main
VAR
    b : BOOL;
    i : DINT;
    r : REAL;
    s : STRING;
    t : TIME;
END_VAR

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_NUMBER{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_NUMBER{{.*}}
s := MOD(s, s);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_NUMBER{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_NUMBER{{.*}}
t := MOD(IN1 := t, IN2 := t);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
r := AND(r, r);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT{{.*}}
s := OR(s, s);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT{{.*}}
t := XOR(t, t);

// CHECK-NOT: {{.*}}BOOL is no{{.*}}
// CHECK-NOT: {{.*}}DINT is no{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
// CHECK-NOT: {{.*}}BOOL is no{{.*}}
// CHECK-NOT: {{.*}}DINT is no{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
// CHECK-NOT: {{.*}}BOOL is no{{.*}}
// CHECK-NOT: {{.*}}DINT is no{{.*}}
b := AND(b, r);
i := XOR(i, i, r);
END_FUNCTION
