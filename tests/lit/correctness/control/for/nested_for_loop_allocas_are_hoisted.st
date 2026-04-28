// RUN: (%COMPILE -Onone %s && %RUN) | %CHECK %s
//
// Regression test for stack growth caused by lowered loop temporaries.
// Nested FOR loops are desugared into nested WHILE TRUE loops with synthetic
// locals, e.g. roughly:
//
//   alloca ran_once_1: BOOL
//   alloca is_incrementing_1: BOOL
//   i := 1
//   is_incrementing_1 := TRUE
//   WHILE TRUE DO
//       ...
//       alloca ran_once_0: BOOL
//       alloca is_incrementing_0: BOOL
//       j := 1
//       is_incrementing_0 := TRUE
//       WHILE TRUE DO
//           ...
//       END_WHILE
//   END_WHILE
//
// If those `alloca` statements are emitted in-place, the inner loop temps are
// allocated once per outer-loop iteration and the stack eventually overflows at
// `-Onone`. Codegen must hoist the actual stack slots to the function entry
// block and only keep the initialization stores in place.

FUNCTION main : DINT
    VAR
        i, j : DINT;
        counter : DINT;
    END_VAR

    counter := 0;

    FOR i := 1 TO 1000000 DO
        FOR j := 1 TO 1 DO
            counter := counter + 1;
        END_FOR
    END_FOR

    printf('counter=%d$N', counter); // CHECK: counter=1000000
    main := 0;
END_FUNCTION
