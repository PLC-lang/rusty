// Concrete implementer in its own compilation unit. After AggregateTypeLowerer
// runs, the call to make_label inside main.st must resolve against the
// rewritten signature — if the closure misses the implementer's unit while
// rewriting the interface, the dispatcher's vtable entry and the concrete
// implementation disagree about return type and the runtime garbles the
// STRING result.

FUNCTION_BLOCK Labeller IMPLEMENTS ILabeller
    METHOD make_label : STRING[16]
        VAR_INPUT
            n : DINT;
        END_VAR
        IF n = 0 THEN
            make_label := 'zero';
        ELSE
            make_label := 'item';
        END_IF
    END_METHOD
END_FUNCTION_BLOCK
