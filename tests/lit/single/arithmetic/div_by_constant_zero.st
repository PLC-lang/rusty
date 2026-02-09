// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
VAR_GLOBAL CONSTANT
    ConstantZero: DINT := 0;
END_VAR

FUNCTION main
    VAR
        x : DINT;
        y : DINT;
        divX : DINT;
        divY : DINT;
    END_VAR

    x := 5;
    y := 0;

    divX := x / ConstantZero;
    divY := x / y;

    printf('%d$N', divX);
    printf('%d$N', divY);
END_FUNCTION
