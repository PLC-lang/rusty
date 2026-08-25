// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// An array of structs that hold an array of structs must accept a nested initializer.
// The parenthesized struct initializers have to stay recognizable as such after the
// initializer is const-folded into the generated constructor.

TYPE
    Inner : STRUCT
        value : DINT;
    END_STRUCT;

    Outer : STRUCT
        arr : ARRAY[0..1] OF Inner;
    END_STRUCT;
END_TYPE

VAR_GLOBAL
    gArr : ARRAY[0..1] OF Outer := [(arr := [(value := 50), (value := 60)]),
                                    (arr := [(value := 70), (value := 80)])];
END_VAR

FUNCTION main : DINT
VAR
    varArr : ARRAY[0..1] OF Outer := [(arr := [(value := 10), (value := 20)]),
                                      (arr := [(value := 30), (value := 40)])];
END_VAR
    // CHECK: varArr=10,20,30,40
    printf('varArr=%d,%d,%d,%d$N', varArr[0].arr[0].value, varArr[0].arr[1].value,
                                   varArr[1].arr[0].value, varArr[1].arr[1].value);

    // CHECK: gArr=50,60,70,80
    printf('gArr=%d,%d,%d,%d$N', gArr[0].arr[0].value, gArr[0].arr[1].value,
                                 gArr[1].arr[0].value, gArr[1].arr[1].value);

    main := 0;
END_FUNCTION
