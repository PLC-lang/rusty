// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
//
// MOD takes numbers and durations, AND, OR, XOR and NOT take bit and integer values. Every other argument
// is rejected with E062, the same for the implicit and the explicit (named) call form. In a call that
// mixes types, only the argument of the wrong type is reported. MOD on a duration needs a duration
// of the same type as its other argument, any other pair is rejected with E156.

FUNCTION main
VAR
    b : BOOL;
    i : DINT;
    r : REAL;
    s : STRING;
    t : TIME;
    lt : LTIME;
    d : DATE;
END_VAR

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION{{.*}}
s := MOD(s, s);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION{{.*}}
d := MOD(IN1 := d, IN2 := d);

// CHECK: {{.*}}error[E156]{{.*}}Operator `MOD` is not defined for `TIME` and `LTIME`{{.*}}
t := MOD(t, lt);

// CHECK: {{.*}}error[E156]{{.*}}Operator `MOD` is not defined for `DINT` and `TIME`{{.*}}
i := MOD(IN2 := t, IN1 := i);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
r := AND(r, r);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT{{.*}}
s := OR(s, s);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT{{.*}}
// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT{{.*}}
t := XOR(t, t);

// CHECK: {{.*}}error[E062]{{.*}}Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT{{.*}}
// CHECK-NOT: {{.*}}REAL is no ANY_BIT{{$}}
r := NOT(IN := r);

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
