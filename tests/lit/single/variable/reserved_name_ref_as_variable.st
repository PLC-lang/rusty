// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for #1605: a local variable named `ref` must not be shadowed by the `REF` builtin
// when used as the base of a qualified call like `ref.foo()`.

FUNCTION_BLOCK MyFb
    METHOD foo
        printf('MyFb::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    ref : MyFb;
END_VAR
    ref.foo(); // CHECK: MyFb::foo
END_FUNCTION
