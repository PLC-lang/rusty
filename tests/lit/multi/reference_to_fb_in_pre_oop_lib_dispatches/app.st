{external}
FUNCTION_BLOCK FB_B
END_FUNCTION_BLOCK

{external}
FUNCTION_BLOCK FB_A
VAR
    target : REFERENCE TO FB_B;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_B_DERIVED EXTENDS FB_B
    printf('FB_B_DERIVED body$N');
END_FUNCTION_BLOCK

FUNCTION main
VAR
    a       : FB_A;
    base    : FB_B;
    derived : FB_B_DERIVED;
END_VAR

a.target REF= base;
a();

a.target REF= derived;
a();
END_FUNCTION
