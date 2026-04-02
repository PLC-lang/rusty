FUNCTION main : DINT
VAR
    result : DINT;
END_VAR
result := helper();
printf('%d$N', result);
printf('%d$N', doubler(result));
main := 0;
END_FUNCTION
