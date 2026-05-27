// Interface in its own compilation unit. The method returns an aggregate
// (STRING[16]) — AggregateTypeLowerer rewrites this method's signature to
// take the return as a VAR_IN_OUT parameter. The interface's signature
// change must propagate through the closure to every unit that dispatches
// through ILabeller — `impl.st`, `dispatcher.st`, and `main.st`.

INTERFACE ILabeller
    METHOD make_label : STRING[16]
        VAR_INPUT
            n : DINT;
        END_VAR
    END_METHOD
END_INTERFACE
