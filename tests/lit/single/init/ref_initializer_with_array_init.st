// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    globalString: STRING := 'Global String';
END_VAR

FUNCTION main : DINT
VAR
    localString: REF_TO STRING := REF(globalString);
    arr: ARRAY[1..2] OF DINT := [1, 2];
END_VAR
    printf('ref=%s$N', localString); // CHECK: ref=Global String
    printf('arr1=%d$N', arr[1]); // CHECK: arr1=1
    printf('arr2=%d$N', arr[2]); // CHECK: arr2=2
    main := 0;
END_FUNCTION
