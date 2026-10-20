// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: An instance whose declared type is a base (`THIS^`, `ptr^`, a `REFERENCE TO`) is converted to an
// interface, by assignment or as an argument. The fat pointer must carry the itable of the actual instance,
// so a derived override runs. This also holds for interfaces added at a derived level, for an upcast to a
// parent interface after the conversion, and for classes. A plain instance keeps the itable of its type.

INTERFACE ISpeaker
    METHOD say : DINT
    END_METHOD
END_INTERFACE

INTERFACE ILoud EXTENDS ISpeaker
    METHOD shout : DINT
    END_METHOD
END_INTERFACE

INTERFACE ITagged
    METHOD tag : DINT
    END_METHOD
END_INTERFACE

FUNCTION speak : DINT
    VAR_INPUT
        s : ISpeaker;
    END_VAR

    speak := s.say();
END_FUNCTION

FUNCTION_BLOCK Dog IMPLEMENTS ILoud
    METHOD say : DINT
        say := 1;
    END_METHOD

    METHOD shout : DINT
        shout := 10;
    END_METHOD

    METHOD viaAssignment : DINT
        VAR
            s : ISpeaker;
        END_VAR

        s := THIS^;
        viaAssignment := s.say();
    END_METHOD

    METHOD viaArgument : DINT
        viaArgument := speak(THIS^);
    END_METHOD

    METHOD viaUpcast : DINT
        VAR
            l : ILoud;
            s : ISpeaker;
        END_VAR

        l := THIS^;
        s := l;
        viaUpcast := l.shout() + s.say();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Puppy EXTENDS Dog IMPLEMENTS ITagged
    METHOD say : DINT
        say := 2;
    END_METHOD

    METHOD tag : DINT
        tag := 200;
    END_METHOD

    METHOD viaDerivedInterface : DINT
        VAR
            t : ITagged;
        END_VAR

        t := THIS^;
        viaDerivedInterface := t.tag();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Pup EXTENDS Puppy
    METHOD say : DINT
        say := 3;
    END_METHOD

    METHOD shout : DINT
        shout := 30;
    END_METHOD

    METHOD tag : DINT
        tag := 300;
    END_METHOD
END_FUNCTION_BLOCK

CLASS Cat IMPLEMENTS ISpeaker
    METHOD say : DINT
        say := 4;
    END_METHOD
END_CLASS

CLASS Kitten EXTENDS Cat
    METHOD say : DINT
        say := 5;
    END_METHOD
END_CLASS

FUNCTION main : DINT
    VAR
        d : Dog;
        p : Puppy;
        g : Pup;
        k : Kitten;
        s : ISpeaker;
        pd : POINTER TO Dog;
        rd : REFERENCE TO Dog;
        pc : POINTER TO Cat;
        dogs : ARRAY[1..2] OF POINTER TO Dog;
    END_VAR

    // CHECK: assignment: 1 2 3
    printf('assignment: %d %d %d$N', d.viaAssignment(), p.viaAssignment(), g.viaAssignment());

    // CHECK: argument: 1 2 3
    printf('argument: %d %d %d$N', d.viaArgument(), p.viaArgument(), g.viaArgument());

    // CHECK: upcast: 11 12 33
    printf('upcast: %d %d %d$N', d.viaUpcast(), p.viaUpcast(), g.viaUpcast());

    // CHECK: derived interface: 200 300
    printf('derived interface: %d %d$N', p.viaDerivedInterface(), g.viaDerivedInterface());

    // CHECK: pointer: 3 3
    pd := ADR(g);
    s := pd^;
    printf('pointer: %d %d$N', s.say(), speak(pd^));

    // CHECK: reference: 2 2
    rd REF= p;
    s := rd;
    printf('reference: %d %d$N', s.say(), speak(rd));

    // CHECK: array of pointers: 1 3
    dogs[1] := ADR(d);
    dogs[2] := ADR(g);
    printf('array of pointers: %d %d$N', speak(dogs[1]^), speak(dogs[2]^));

    // CHECK: class: 5
    pc := ADR(k);
    s := pc^;
    printf('class: %d$N', s.say());

    // CHECK: exact: 1 2
    s := d;
    printf('exact: %d %d$N', s.say(), speak(p));
END_FUNCTION
