FUNCTION_BLOCK Device
    VAR
        channel: Channel;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    channel: Channel;
    device: Device;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
END_FUNCTION
