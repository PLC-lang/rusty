// RUN: %COMPILE %s && %RUN | %CHECK %s
// Chains of date and time arithmetic

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

END_FUNCTION
