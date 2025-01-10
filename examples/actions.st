PROGRAM main
VAR
    a,b : DINT;
END_VAR

action1();
action2();

END_PROGRAM

ACTIONS
    ACTION action1
        a := a + 1;
    END_ACTION
    ACTION action2
        b := b + 2;
    END_ACTION
END_ACTIONS
