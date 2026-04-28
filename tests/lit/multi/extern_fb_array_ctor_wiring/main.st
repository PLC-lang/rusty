FUNCTION main : DINT
VAR
    fb : myFb;
END_VAR
    // Array elements should be initialized by the ctor
    printf('%d$N', fb.arr[0]);
    printf('%d$N', fb.arr[2]);
    printf('%d$N', fb.arr[4]);
    // Scalar should also be initialized
    printf('%d$N', fb.x);
    main := 0;
END_FUNCTION
