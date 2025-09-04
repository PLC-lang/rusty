// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION func : int
  VAR_INPUT
    in : INT := 42;
  END_VAR
  func := in;
END_FUNCTION

FUNCTION main : DINT

    PRINTF('func complete:%d$N', func(13)); //CHECK: func complete:13
    PRINTF('func incomplete:%d$N', func()); //CHECK: func incomplete:42

END_FUNCTION

