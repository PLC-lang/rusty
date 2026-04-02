FUNCTION_BLOCK FbConsumer
    VAR
        stored : IFoo;
    END_VAR

    METHOD setRef
        VAR_INPUT
            iface : IFoo;
        END_VAR
        stored := iface;
    END_METHOD

    METHOD invoke : DINT
        invoke := stored.bar();
    END_METHOD
END_FUNCTION_BLOCK
