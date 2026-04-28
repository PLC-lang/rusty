// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo
   VAR_INPUT
       in_literal: ARRAY[1..5] OF DINT;
       in_variable: ARRAY[1..5] OF DINT;
   END_VAR
   printf('in_literal: %d, %d, %d, %d, %d$N', in_literal[1], in_literal[2], in_literal[3], in_literal[4], in_literal[5]);
END_FUNCTION

FUNCTION main
   VAR
       arr: ARRAY[1..5] OF DINT := [11, 12, 13, 14, 15];
   END_VAR

   //CHECK: in_literal: 1, 2, 3, 4, 5
   foo([1, 2, 3, 4, 5], arr);
END_FUNCTION
