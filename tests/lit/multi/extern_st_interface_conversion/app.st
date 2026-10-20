// Redeclare the library interface, function, and function block so the compiler knows their shape
INTERFACE ISpeaker
    METHOD say : DINT
    END_METHOD
END_INTERFACE

INTERFACE ITagged
    METHOD tag : DINT
    END_METHOD
END_INTERFACE

{external}
FUNCTION speak : DINT
    VAR_INPUT
        s : ISpeaker;
    END_VAR
END_FUNCTION

{external}
FUNCTION_BLOCK Dog IMPLEMENTS ISpeaker
    METHOD say : DINT
    END_METHOD

    METHOD describe : DINT
    END_METHOD
END_FUNCTION_BLOCK

// Defined in the application: overrides a library method and adds an interface
FUNCTION_BLOCK Puppy EXTENDS Dog IMPLEMENTS ITagged
    METHOD say : DINT
        say := 2;
    END_METHOD

    METHOD tag : DINT
        tag := 200;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        d : Dog;
        p : Puppy;
        pd : POINTER TO Dog;
        s : ISpeaker;
        t : ITagged;
    END_VAR

    // Library code converts THIS^ and reads the itable from the vtable that the application built
    printf('describe: %d %d$N', d.describe(), p.describe());

    // Application code converts a base pointer and calls into the library through the interface
    pd := ADR(p);
    s := pd^;
    printf('pointer: %d %d$N', s.say(), speak(pd^));

    t := p;
    printf('tag: %d$N', t.tag());
END_FUNCTION
