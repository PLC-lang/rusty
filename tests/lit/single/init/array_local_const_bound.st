// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// A POU-local `VAR CONSTANT` used as an array bound. The generated element constructor is a
// POU of its own, so a bound naming a member of the declaring POU does not resolve inside it
// and has to be taken from the index as a literal. Regression test for PRG-4730: element
// types that need construction (sized strings, structs) previously failed to generate.
//
// The last element of every array is checked as well as the first, so a bound that resolved
// to the wrong value would be caught rather than only a bound that failed to compile.
TYPE Point : STRUCT
    x : DINT := 11;
    y : DINT := 22;
END_STRUCT END_TYPE

PROGRAM prog
VAR CONSTANT
    five : DINT := 5;
    two  : DINT := 2;
END_VAR
VAR
    strings : ARRAY[1..five] OF STRING[63];
    points  : ARRAY[1..five] OF Point;
    multi   : ARRAY[1..two, 1..five] OF Point;
    nested  : ARRAY[1..two] OF ARRAY[1..five] OF Point;
END_VAR
    strings[1] := 'first';
    strings[five] := 'last';

    printf('%s$N', REF(strings[1]));      // CHECK: first
    printf('%s$N', REF(strings[five]));   // CHECK: last

    // Struct defaults must reach every element, including the last one the bound admits.
    printf('%d$N', points[1].x);          // CHECK: 11
    printf('%d$N', points[five].y);       // CHECK: 22

    printf('%d$N', multi[1, 1].x);        // CHECK: 11
    printf('%d$N', multi[two, five].y);   // CHECK: 22

    printf('%d$N', nested[1][1].x);       // CHECK: 11
    printf('%d$N', nested[two][five].y);  // CHECK: 22
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
