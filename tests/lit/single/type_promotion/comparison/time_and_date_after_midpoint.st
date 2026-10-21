// RUN: %COMPILE %s && %RUN | %CHECK %s
// TIME, DATE and DT are signed 64-bit nanosecond values, so values beyond 2^31 milliseconds or seconds compare correctly.
FUNCTION main : DINT
    VAR
        long_time : TIME := T#25d;
        short_time : TIME := T#1h;
        later : DT := DT#2038-01-19-03:14:08;
        earlier : DT := DT#2038-01-19-03:14:07;
        future : DATE := D#2040-01-01;
        past : DATE := D#2020-01-01;
    END_VAR

    // TIME above about 24.8 days
    printf('T#25d > T#1h = %d$N', long_time > short_time);  // CHECK: T#25d > T#1h = 1
    printf('T#1h < T#25d = %d$N', short_time < long_time);  // CHECK-NEXT: T#1h < T#25d = 1
    printf('T#24d21h > T#24d20h = %d$N', T#24d21h > T#24d20h);  // CHECK-NEXT: T#24d21h > T#24d20h = 1

    // DT and DATE after 2038-01-19 03:14:07
    printf('later > earlier = %d$N', later > earlier);      // CHECK-NEXT: later > earlier = 1
    printf('2040 > 2020 = %d$N', future > past);            // CHECK-NEXT: 2040 > 2020 = 1
    printf('2020 >= 2040 = %d$N', past >= future);          // CHECK-NEXT: 2020 >= 2040 = 0
END_FUNCTION
