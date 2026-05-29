FUNCTION main : DINT
VAR
    a       : FB_A;
    derived : FB_B_DERIVED;
END_VAR

counter := 0;

// Invokes FB_A's body, which in turn calls `helper()` on a member of
// static type `FB_B`. Must run FB_B's body (+100), NOT FB_B_DERIVED's
// (+9000), regardless of whether the dispatch pass walked this AST.
a();

// CHECK: counter after a() = 100
printf('counter after a() = %d$N', counter);

// Sanity check: FB_B_DERIVED's body really does add 9000 when invoked
// directly on a `FB_B_DERIVED` instance.
derived();

// CHECK: counter after derived() = 9100
printf('counter after derived() = %d$N', counter);
END_FUNCTION
