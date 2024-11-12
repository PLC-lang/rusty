FUNCTION TO_TIME<T: ANY> : TIME
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_TIME__TIME : TIME
    VAR_INPUT
        in : TIME;
    END_VAR

    TO_TIME__TIME := in;
END_FUNCTION

FUNCTION TO_TIME__LTIME : TIME
    VAR_INPUT
        in : LTIME;
    END_VAR

    TO_TIME__LTIME := LTIME_TO_TIME(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_LTIME<T: ANY> : LTIME
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_LTIME__LTIME : LTIME
    VAR_INPUT
        in : T;
    END_VAR

    TO_LTIME__LTIME := in;
END_FUNCTION

FUNCTION TO_LTIME__TIME : LTIME
    VAR_INPUT
        in : LTIME;
    END_VAR

    TO_LTIME__TIME := TIME_TO_LTIME(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_DT<T: ANY> : DT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_DT__DATE_AND_TIME : DT
    VAR_INPUT
        in : LDT;
    END_VAR

    TO_DT__DATE_AND_TIME := LDT_TO_DT(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_DATE<T: ANY> : DATE
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_DATE__DATE_AND_TIME : DATE
    VAR_INPUT
        in : LDT;
    END_VAR

    TO_DATE__DATE_AND_TIME := LDT_TO_DATE(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_TOD<T: ANY> : TOD
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_TOD__DATE_AND_TIME : TOD
    VAR_INPUT
        in : LDT;
    END_VAR

    TO_TOD__DATE_AND_TIME := DT_TO_TOD(in);
END_FUNCTION

FUNCTION TO_TOD__TIME_OF_DAY : TOD
    VAR_INPUT
        in : LTOD;
    END_VAR

    TO_TOD__TIME_OF_DAY := LTOD_TO_TOD(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_LTOD<T: ANY> : LTOD
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_LTOD__DATE_AND_TIME : LTOD
    VAR_INPUT
        in : LDT;
    END_VAR

    TO_LTOD__DATE_AND_TIME := LDT_TO_LTOD(in);
END_FUNCTION

FUNCTION TO_LTOD__TIME_OF_DAY : LTOD
    VAR_INPUT
        in : TOD;
    END_VAR

    TO_LTOD__TIME_OF_DAY := TOD_TO_LTOD(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_LDT<T: ANY> : LDT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_LDT__DATE_AND_TIME : LDT
    VAR_INPUT
        in : DT;
    END_VAR

    TO_LDT__DATE_AND_TIME := DT_TO_LDT(in);
END_FUNCTION