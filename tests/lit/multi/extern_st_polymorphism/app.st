// Redeclare interface so the compiler knows the shape
INTERFACE IGreeter
    METHOD greet
    END_METHOD
END_INTERFACE

// External declaration — the real implementation is in libpoly.so
{external}
FUNCTION_BLOCK FbA IMPLEMENTS IGreeter
VAR
    id : DINT;
END_VAR
    METHOD greet
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
VAR
    inst : FbA;
    pInst : POINTER TO FbA;
    iface : IGreeter;
END_VAR
    inst.id := 7;
    pInst := ADR(inst);

    // Dynamic vtable dispatch
    pInst^.greet();

    // Interface dispatch (itable)
    iface := inst;
    iface.greet();
END_FUNCTION
