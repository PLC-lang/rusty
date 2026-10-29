// RUN: %COMPILE %s && %RUN | %CHECK %s

// The base of a cast names a type, also when a variant of another enum has the same name.
// `Error` is both the type below and a variant of `Status`.
TYPE Status : (Ok := 0, Error := 1);
END_TYPE

TYPE Error : (NoError := 5, Timeout := 6);
END_TYPE

TYPE Rec : STRUCT
    e : Error;
END_STRUCT
END_TYPE

FUNCTION_BLOCK FB
VAR_OUTPUT
    e : Error;
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    rc : Rec;
    fb : FB;
    s : Status;
    e : Error;
END_VAR
    // CHECK: default 5 5
    printf('default %d %d$N', rc.e, fb.e);

    s := Status#Error;
    e := Error#Timeout;
    // CHECK: cast 1 6
    printf('cast %d %d$N', s, e);
END_FUNCTION
