// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// A POU-local `VAR CONSTANT` is a constant like any other and must be usable everywhere a
// constant is allowed in a type: as a string length, as an array bound, in an expression
// forming either, and as the bound of an array whose element type is itself sized by one.
// Companion to array_local_const_bound.st, which covers element construction.
FUNCTION main : DINT
VAR CONSTANT
    len   : DINT := 16;
    lo    : DINT := 2;
    hi    : DINT := 4;
END_VAR
VAR
    // string length from a local constant, and from an expression over one
    sized      : STRING[len];
    computed   : STRING[len * 2];
    wide       : WSTRING[len];

    // array bounds from local constants, including a non-zero lower bound
    bounded    : ARRAY[lo..hi] OF DINT;
    expression : ARRAY[1..len / 4] OF DINT;

    // both at once: bound and element length from local constants
    both       : ARRAY[lo..hi] OF STRING[len];
END_VAR
    sized := 'sixteen chars ..';
    computed := 'thirty two characters, exactly!!';
    bounded[lo] := 100;
    bounded[hi] := 400;
    expression[1] := 1;
    expression[len / 4] := 4;
    both[lo] := 'low';
    both[hi] := 'high';

    printf('%s$N', REF(sized));            // CHECK: sixteen chars ..
    printf('%s$N', REF(computed));         // CHECK: thirty two characters, exactly!!

    printf('%d$N', bounded[lo]);           // CHECK: 100
    printf('%d$N', bounded[hi]);           // CHECK: 400

    printf('%d$N', expression[1]);         // CHECK: 1
    printf('%d$N', expression[len / 4]);   // CHECK: 4

    printf('%s$N', REF(both[lo]));         // CHECK: low
    printf('%s$N', REF(both[hi]));         // CHECK: high

    main := 0;
END_FUNCTION
