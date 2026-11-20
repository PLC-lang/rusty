
(********************
*
* Converts a floating point number to SINT
*
********************)
FUNCTION TRUNC_SINT<T: ANY_REAL> : SINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to SINT
*
********************)
FUNCTION TRUNC_SINT__REAL : SINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_SINT__REAL := REAL_TRUNC_SINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to SINT
*
********************)
FUNCTION TRUNC_SINT__LREAL : SINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_SINT__LREAL := LREAL_TRUNC_SINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to USINT
*
********************)
FUNCTION TRUNC_USINT<T: ANY_REAL> : USINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to USINT
*
********************)
FUNCTION TRUNC_USINT__REAL : USINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_USINT__REAL := REAL_TRUNC_USINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to USINT
*
********************)
FUNCTION TRUNC_USINT__LREAL : USINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_USINT__LREAL := LREAL_TRUNC_USINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to INT
*
********************)
FUNCTION TRUNC_INT<T: ANY_REAL> : INT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to INT
*
********************)
FUNCTION TRUNC_INT__REAL : INT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_INT__REAL := REAL_TRUNC_INT(in);
END_FUNCTION


(********************
*
* Converts LREAL to INT
*
********************)
FUNCTION TRUNC_INT__LREAL : INT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_INT__LREAL := LREAL_TRUNC_INT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to UINT
*
********************)
FUNCTION TRUNC_UINT<T: ANY_REAL> : UINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to UINT
*
********************)
FUNCTION TRUNC_UINT__REAL : UINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_UINT__REAL := REAL_TRUNC_UINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to UINT
*
********************)
FUNCTION TRUNC_UINT__LREAL : UINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_UINT__LREAL := LREAL_TRUNC_UINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to DINT
*
********************)
FUNCTION TRUNC_DINT<T: ANY_REAL> : DINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to DINT
*
********************)
FUNCTION TRUNC_DINT__REAL : DINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_DINT__REAL := REAL_TRUNC_DINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to DINT
*
********************)
FUNCTION TRUNC_DINT__LREAL : DINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_DINT__LREAL := LREAL_TRUNC_DINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to UDINT
*
********************)
FUNCTION TRUNC_UDINT<T: ANY_REAL> : UDINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to UDINT
*
********************)
FUNCTION TRUNC_UDINT__REAL : UDINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_UDINT__REAL := REAL_TRUNC_UDINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to UDINT
*
********************)
FUNCTION TRUNC_UDINT__LREAL : UDINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_UDINT__LREAL := LREAL_TRUNC_UDINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to LINT
*
********************)
FUNCTION TRUNC_LINT<T: ANY_REAL> : LINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to LINT
*
********************)
FUNCTION TRUNC_LINT__REAL : LINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_LINT__REAL := REAL_TRUNC_LINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to LINT
*
********************)
FUNCTION TRUNC_LINT__LREAL : LINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_LINT__LREAL := LREAL_TRUNC_LINT(in);
END_FUNCTION


(********************
*
* Converts a floating point number to ULINT
*
********************)
FUNCTION TRUNC_ULINT<T: ANY_REAL> : ULINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION


(********************
*
* Converts REAL to ULINT
*
********************)
FUNCTION TRUNC_ULINT__REAL : ULINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TRUNC_ULINT__REAL := REAL_TRUNC_ULINT(in);
END_FUNCTION


(********************
*
* Converts LREAL to ULINT
*
********************)
FUNCTION TRUNC_ULINT__LREAL : ULINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TRUNC_ULINT__LREAL := LREAL_TRUNC_ULINT(in);
END_FUNCTION

