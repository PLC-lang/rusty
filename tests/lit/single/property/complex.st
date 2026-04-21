// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
  x : DINT;
  foo: nest_level1;
  END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK nest_level1
VAR
  foo: nest_level2;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK nest_level2
VAR
  foo: DINT := 0;
END_VAR
  PROPERTY myProp : DINT
    GET
      myProp := foo + 5;
    END_GET
    SET
      foo := myProp + foo;
      END_SET
  END_PROPERTY
END_FUNCTION_BLOCK


FUNCTION main
    VAR
        bar: fb;
        x : DINT := 1;
    END_VAR

printf('%d$N', bar.foo.foo.myProp);   // CHECK: 5
bar.foo.foo.myProp := 10;
printf('%d$N', bar.foo.foo.myProp);   // CHECK: 15
END_FUNCTION
