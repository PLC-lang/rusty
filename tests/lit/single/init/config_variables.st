// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK FB 
VAR 
  foo AT %I* : DINT; 
END_VAR
    printf('%d$N', foo);
END_FUNCTION_BLOCK

VAR_CONFIG
  prog.instance1.foo AT %IX1.2.1 : DINT;
  prog.instance2.foo AT %QX1.2.2 : DINT;
END_VAR

PROGRAM prog 
VAR
    instance1: FB;
    instance2: FB;
END_VAR
    instance1(); // CHECK: 23
    instance2(); // CHECK: 48
END_PROGRAM

FUNCTION main : DINT
    __init___config_variables_st();

    %IX1.2.1 := 23;
    %QX1.2.2 := 48;
    prog();
END_FUNCTION
