// RUN: %COMPILE %s && %RUN | %CHECK %s

// An enum variable without an initializer starts with the enum's default, the first
// member here, also when it is a struct member, an array element or a member of a
// function block, and not only when it is a plain local, global or program variable.
TYPE Col : (r := 3, g := 7);
END_TYPE

TYPE ColAlias : Col;
END_TYPE

TYPE ColArr : ARRAY[0..1] OF Col;
END_TYPE

TYPE Rec : STRUCT
    c : Col;
    explicit : Col := g;
    alias : ColAlias;
    inline : (x := 4, y := 5);
END_STRUCT
END_TYPE

FUNCTION_BLOCK FB
VAR_INPUT
    i : Col;
END_VAR
VAR_OUTPUT
    o : Col;
END_VAR
VAR
    c : Col;
END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    grc : Rec;
    gac : ARRAY[0..1] OF Col;
    gfb : FB;
END_VAR

PROGRAM prg
VAR
    rc : Rec;
    ac : ARRAY[0..1] OF Col;
    fb : FB;
END_VAR
END_PROGRAM

FUNCTION name_of : STRING
VAR_INPUT
    c : Col;
END_VAR
    CASE c OF
        Col#r: name_of := 'r';
        Col#g: name_of := 'g';
    ELSE
        name_of := 'none';
    END_CASE
END_FUNCTION

FUNCTION main : DINT
VAR
    lc : Col;
    rc : Rec;
    ac : ARRAY[0..1] OF Col;
    ap : ARRAY[0..2] OF Col := [g];
    an : ColArr;
    fb : FB;
    s1, s2 : STRING;
END_VAR
    // CHECK: local 3
    printf('local %d$N', lc);
    // CHECK: struct 3 7 3 4
    printf('struct %d %d %d %d$N', rc.c, rc.explicit, rc.alias, rc.inline);
    // CHECK: array 3 3
    printf('array %d %d$N', ac[0], ac[1]);
    // CHECK: partial array 7 3 3
    printf('partial array %d %d %d$N', ap[0], ap[1], ap[2]);
    // CHECK: named array 3 3
    printf('named array %d %d$N', an[0], an[1]);
    // CHECK: fb 3 3
    printf('fb %d %d$N', fb.i, fb.o);
    // CHECK: global 3 3 3 3
    printf('global %d %d %d %d$N', grc.c, gac[1], gfb.i, gfb.o);
    // CHECK: program 3 3 3
    printf('program %d %d %d$N', prg.rc.c, prg.ac[1], prg.fb.i);
    // CHECK: compare TRUE
    IF rc.c = Col#r THEN
        printf('compare TRUE$N');
    ELSE
        printf('compare FALSE$N');
    END_IF
    // CHECK: case r r
    s1 := name_of(rc.c);
    s2 := name_of(ac[1]);
    printf('case %s %s$N', REF(s1), REF(s2));
END_FUNCTION
