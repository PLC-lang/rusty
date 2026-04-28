INTERFACE IGreeter
    METHOD greet
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK LocalFb IMPLEMENTS IGreeter
VAR
    id : DINT;
END_VAR
    METHOD greet
        printf('LocalFb::greet (id=%d)$N', THIS^.id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
VAR
    ext : ExternalFb;
    loc : LocalFb;
    pExt : POINTER TO ExternalFb;
    pLoc : POINTER TO LocalFb;
    iface : IGreeter;
END_VAR
    loc.id := 99;
    pExt := ADR(ext);
    pLoc := ADR(loc);

    // Dynamic vtable dispatch
    pExt^.greet();
    pLoc^.greet();

    // Interface dispatch (itable)
    iface := ext;
    iface.greet();

    iface := loc;
    iface.greet();
END_FUNCTION
