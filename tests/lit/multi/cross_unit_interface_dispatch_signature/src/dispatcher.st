// The dispatcher holds an ILabeller reference and dispatches make_label
// through it. Lives in its own compilation unit so that the reverse-dep
// closure must reach this unit when the interface's method signature
// is rewritten by AggregateTypeLowerer. If the closure misses this unit,
// the call site here keeps the pre-rewrite STRING-return calling
// convention and breaks at runtime.

FUNCTION_BLOCK Dispatcher
    VAR
        target : ILabeller;
    END_VAR

    METHOD bind
        VAR_INPUT
            iface : ILabeller;
        END_VAR
        target := iface;
    END_METHOD

    METHOD label_for : STRING[16]
        VAR_INPUT
            n : DINT;
        END_VAR
        label_for := target.make_label(n);
    END_METHOD
END_FUNCTION_BLOCK
