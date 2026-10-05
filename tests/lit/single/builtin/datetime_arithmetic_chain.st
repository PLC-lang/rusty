// RUN: %COMPILE %s && %RUN | %CHECK %s
// Chains of date and time arithmetic, in operator and builtin form

FUNCTION main
    VAR
        x : DT := DT#2027-05-05-22:30:15;
        y : TIME := TIME#20s;
        z : TOD := TOD#23:58:00;
    END_VAR

    x := x + y + y + TIME#2h555s;
    printf('%s$N', REF(DT_TO_STRING(x)));                                           // CHECK: DT#2027-05-06-00:40:10

    z := z + TIME#2m5s;
    printf('%s$N', REF(TOD_TO_STRING(z)));                                          // CHECK: TOD#00:00:05

    x := ADD(x, y, MUL(y, 2));
    printf('%s$N', REF(DT_TO_STRING(x)));                                           // CHECK: DT#2027-05-06-00:41:10

    z := SUB(z, TIME#10s);
    printf('%s$N', REF(TOD_TO_STRING(z)));                                          // CHECK: TOD#23:59:55

END_FUNCTION
