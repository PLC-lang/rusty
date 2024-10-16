(********************
*
* Converts any other numerical value to SINT
*
*********************)
FUNCTION TO_SINT<T: ANY_NUM> : SINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to SINT
*
*********************)
FUNCTION TO_SINT__SINT : SINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_SINT__SINT := in;
END_FUNCTION

(********************
*
* Converts USINT to SINT
*
*********************)
FUNCTION TO_SINT__USINT : SINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_SINT__USINT := USINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts INT to SINT
*
*********************)
FUNCTION TO_SINT__INT : SINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_SINT__INT := INT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts UINT to SINT
*
*********************)
FUNCTION TO_SINT__UINT : SINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_SINT__UINT := UINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts DINT to SINT
*
*********************)
FUNCTION TO_SINT__DINT : SINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_SINT__DINT := DINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to SINT
*
*********************)
FUNCTION TO_SINT__UDINT : SINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_SINT__UDINT := UDINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts LINT to SINT
*
*********************)
FUNCTION TO_SINT__LINT : SINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_SINT__LINT := LINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to SINT
*
*********************)
FUNCTION TO_SINT__ULINT : SINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_SINT__ULINT := ULINT_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to SINT
*
*********************)
FUNCTION TO_SINT__LREAL : SINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_SINT__LREAL := LREAL_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts REAL to SINT
*
*********************)
FUNCTION TO_SINT__REAL : SINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_SINT__REAL := REAL_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to SINT
*
*********************)
FUNCTION TO_SINT__BOOL : SINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_SINT__BOOL := BOOL_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to SINT
*
*********************)
FUNCTION TO_SINT__BYTE : SINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_SINT__BYTE := BYTE_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts WORD to SINT
*
*********************)
FUNCTION TO_SINT__WORD : SINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_SINT__WORD := WORD_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to SINT
*
*********************)
FUNCTION TO_SINT__DWORD : SINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_SINT__DWORD := DWORD_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to SINT
*
*********************)
FUNCTION TO_SINT__LWORD : SINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_SINT__LWORD := LWORD_TO_SINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to USINT
*
*********************)
FUNCTION TO_USINT<T: ANY_NUM> : USINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to USINT
*
*********************)
FUNCTION TO_USINT__SINT : USINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_USINT__SINT := SINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts USINT to USINT
*
*********************)
FUNCTION TO_USINT__USINT : USINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_USINT__USINT := in;
END_FUNCTION

(********************
*
* Converts INT to USINT
*
*********************)
FUNCTION TO_USINT__INT : USINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_USINT__INT := INT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts UINT to USINT
*
*********************)
FUNCTION TO_USINT__UINT : USINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_USINT__UINT := UINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts DINT to USINT
*
*********************)
FUNCTION TO_USINT__DINT : USINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_USINT__DINT := DINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to USINT
*
*********************)
FUNCTION TO_USINT__UDINT : USINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_USINT__UDINT := UDINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts LINT to USINT
*
*********************)
FUNCTION TO_USINT__LINT : USINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_USINT__LINT := LINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to USINT
*
*********************)
FUNCTION TO_USINT__ULINT : USINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_USINT__ULINT := ULINT_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to USINT
*
*********************)
FUNCTION TO_USINT__LREAL : USINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_USINT__LREAL := LREAL_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts REAL to USINT
*
*********************)
FUNCTION TO_USINT__REAL : USINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_USINT__REAL := REAL_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to USINT
*
*********************)
FUNCTION TO_USINT__BOOL : USINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_USINT__BOOL := BOOL_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to USINT
*
*********************)
FUNCTION TO_USINT__BYTE : USINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_USINT__BYTE := BYTE_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts WORD to USINT
*
*********************)
FUNCTION TO_USINT__WORD : USINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_USINT__WORD := WORD_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to USINT
*
*********************)
FUNCTION TO_USINT__DWORD : USINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_USINT__DWORD := DWORD_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to USINT
*
*********************)
FUNCTION TO_USINT__LWORD : USINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_USINT__LWORD := LWORD_TO_USINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to INT
*
*********************)
FUNCTION TO_INT<T: ANY_NUM> : INT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to INT
*
*********************)
FUNCTION TO_INT__SINT : INT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_INT__SINT := SINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts USINT to INT
*
*********************)
FUNCTION TO_INT__USINT : INT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_INT__USINT := USINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts INT to INT
*
*********************)
FUNCTION TO_INT__INT : INT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_INT__INT := in;
END_FUNCTION

(********************
*
* Converts UINT to INT
*
*********************)
FUNCTION TO_INT__UINT : INT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_INT__UINT := UINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts DINT to INT
*
*********************)
FUNCTION TO_INT__DINT : INT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_INT__DINT := DINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts UDINT to INT
*
*********************)
FUNCTION TO_INT__UDINT : INT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_INT__UDINT := UDINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts LINT to INT
*
*********************)
FUNCTION TO_INT__LINT : INT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_INT__LINT := LINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts ULINT to INT
*
*********************)
FUNCTION TO_INT__ULINT : INT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_INT__ULINT := ULINT_TO_INT(in);
END_FUNCTION

(********************
*
* Converts LREAL to INT
*
*********************)
FUNCTION TO_INT__LREAL : INT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_INT__LREAL := LREAL_TO_INT(in);
END_FUNCTION

(********************
*
* Converts REAL to INT
*
*********************)
FUNCTION TO_INT__REAL : INT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_INT__REAL := REAL_TO_INT(in);
END_FUNCTION

(********************
*
* Converts BOOL to INT
*
*********************)
FUNCTION TO_INT__BOOL : INT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_INT__BOOL := BOOL_TO_INT(in);
END_FUNCTION

(********************
*
* Converts BYTE to INT
*
*********************)
FUNCTION TO_INT__BYTE : INT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_INT__BYTE := BYTE_TO_INT(in);
END_FUNCTION

(********************
*
* Converts WORD to INT
*
*********************)
FUNCTION TO_INT__WORD : INT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_INT__WORD := WORD_TO_INT(in);
END_FUNCTION

(********************
*
* Converts DWORD to INT
*
*********************)
FUNCTION TO_INT__DWORD : INT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_INT__DWORD := DWORD_TO_INT(in);
END_FUNCTION

(********************
*
* Converts LWORD to INT
*
*********************)
FUNCTION TO_INT__LWORD : INT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_INT__LWORD := LWORD_TO_INT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to UINT
*
*********************)
FUNCTION TO_UINT<T: ANY_NUM> : UINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to UINT
*
*********************)
FUNCTION TO_UINT__SINT : UINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_UINT__SINT := SINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts USINT to UINT
*
*********************)
FUNCTION TO_UINT__USINT : UINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_UINT__USINT := USINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts INT to UINT
*
*********************)
FUNCTION TO_UINT__INT : UINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_UINT__INT := INT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts UINT to UINT
*
*********************)
FUNCTION TO_UINT__UINT : UINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_UINT__UINT := in;
END_FUNCTION

(********************
*
* Converts DINT to UINT
*
*********************)
FUNCTION TO_UINT__DINT : UINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_UINT__DINT := DINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to UINT
*
*********************)
FUNCTION TO_UINT__UDINT : UINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_UINT__UDINT := UDINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts LINT to UINT
*
*********************)
FUNCTION TO_UINT__LINT : UINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_UINT__LINT := LINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to UINT
*
*********************)
FUNCTION TO_UINT__ULINT : UINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_UINT__ULINT := ULINT_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to UINT
*
*********************)
FUNCTION TO_UINT__LREAL : UINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_UINT__LREAL := LREAL_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts REAL to UINT
*
*********************)
FUNCTION TO_UINT__REAL : UINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_UINT__REAL := REAL_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to UINT
*
*********************)
FUNCTION TO_UINT__BOOL : UINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_UINT__BOOL := BOOL_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to UINT
*
*********************)
FUNCTION TO_UINT__BYTE : UINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_UINT__BYTE := BYTE_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts WORD to UINT
*
*********************)
FUNCTION TO_UINT__WORD : UINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_UINT__WORD := WORD_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to UINT
*
*********************)
FUNCTION TO_UINT__DWORD : UINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_UINT__DWORD := DWORD_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to UINT
*
*********************)
FUNCTION TO_UINT__LWORD : UINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_UINT__LWORD := LWORD_TO_UINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to DINT
*
*********************)
FUNCTION TO_DINT<T: ANY_NUM> : DINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to DINT
*
*********************)
FUNCTION TO_DINT__SINT : DINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_DINT__SINT := SINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts USINT to DINT
*
*********************)
FUNCTION TO_DINT__USINT : DINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_DINT__USINT := USINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts INT to DINT
*
*********************)
FUNCTION TO_DINT__INT : DINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_DINT__INT := INT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts UINT to DINT
*
*********************)
FUNCTION TO_DINT__UINT : DINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_DINT__UINT := UINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts DINT to DINT
*
*********************)
FUNCTION TO_DINT__DINT : DINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_DINT__DINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to DINT
*
*********************)
FUNCTION TO_DINT__UDINT : DINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_DINT__UDINT := UDINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts LINT to DINT
*
*********************)
FUNCTION TO_DINT__LINT : DINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_DINT__LINT := LINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to DINT
*
*********************)
FUNCTION TO_DINT__ULINT : DINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_DINT__ULINT := ULINT_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to DINT
*
*********************)
FUNCTION TO_DINT__LREAL : DINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_DINT__LREAL := LREAL_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts REAL to DINT
*
*********************)
FUNCTION TO_DINT__REAL : DINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_DINT__REAL := REAL_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to DINT
*
*********************)
FUNCTION TO_DINT__BOOL : DINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_DINT__BOOL := BOOL_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to DINT
*
*********************)
FUNCTION TO_DINT__BYTE : DINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_DINT__BYTE := BYTE_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts WORD to DINT
*
*********************)
FUNCTION TO_DINT__WORD : DINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_DINT__WORD := WORD_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to DINT
*
*********************)
FUNCTION TO_DINT__DWORD : DINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_DINT__DWORD := DWORD_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to DINT
*
*********************)
FUNCTION TO_DINT__LWORD : DINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_DINT__LWORD := LWORD_TO_DINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to UDINT
*
*********************)
FUNCTION TO_UDINT<T: ANY_NUM> : UDINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to UDINT
*
*********************)
FUNCTION TO_UDINT__SINT : UDINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_UDINT__SINT := SINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts USINT to UDINT
*
*********************)
FUNCTION TO_UDINT__USINT : UDINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_UDINT__USINT := USINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts INT to UDINT
*
*********************)
FUNCTION TO_UDINT__INT : UDINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_UDINT__INT := INT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts UINT to UDINT
*
*********************)
FUNCTION TO_UDINT__UINT : UDINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_UDINT__UINT := UINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts DINT to UDINT
*
*********************)
FUNCTION TO_UDINT__DINT : UDINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_UDINT__DINT := DINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to UDINT
*
*********************)
FUNCTION TO_UDINT__UDINT : UDINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_UDINT__UDINT := in;
END_FUNCTION

(********************
*
* Converts LINT to UDINT
*
*********************)
FUNCTION TO_UDINT__LINT : UDINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_UDINT__LINT := LINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to UDINT
*
*********************)
FUNCTION TO_UDINT__ULINT : UDINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_UDINT__ULINT := ULINT_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to UDINT
*
*********************)
FUNCTION TO_UDINT__LREAL : UDINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_UDINT__LREAL := LREAL_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts REAL to UDINT
*
*********************)
FUNCTION TO_UDINT__REAL : UDINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_UDINT__REAL := REAL_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to UDINT
*
*********************)
FUNCTION TO_UDINT__BOOL : UDINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_UDINT__BOOL := BOOL_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to UDINT
*
*********************)
FUNCTION TO_UDINT__BYTE : UDINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_UDINT__BYTE := BYTE_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts WORD to UDINT
*
*********************)
FUNCTION TO_UDINT__WORD : UDINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_UDINT__WORD := WORD_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to UDINT
*
*********************)
FUNCTION TO_UDINT__DWORD : UDINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_UDINT__DWORD := DWORD_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to UDINT
*
*********************)
FUNCTION TO_UDINT__LWORD : UDINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_UDINT__LWORD := LWORD_TO_UDINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to LINT
*
*********************)
FUNCTION TO_LINT<T: ANY_NUM> : LINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to LINT
*
*********************)
FUNCTION TO_LINT__SINT : LINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_LINT__SINT := SINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts USINT to LINT
*
*********************)
FUNCTION TO_LINT__USINT : LINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_LINT__USINT := USINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts INT to LINT
*
*********************)
FUNCTION TO_LINT__INT : LINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_LINT__INT := INT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts UINT to LINT
*
*********************)
FUNCTION TO_LINT__UINT : LINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_LINT__UINT := UINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts DINT to LINT
*
*********************)
FUNCTION TO_LINT__DINT : LINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_LINT__DINT := DINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to LINT
*
*********************)
FUNCTION TO_LINT__UDINT : LINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_LINT__UDINT := UDINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts LINT to LINT
*
*********************)
FUNCTION TO_LINT__LINT : LINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_LINT__LINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to LINT
*
*********************)
FUNCTION TO_LINT__ULINT : LINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_LINT__ULINT := ULINT_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts LREAL to LINT
*
*********************)
FUNCTION TO_LINT__LREAL : LINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_LINT__LREAL := LREAL_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts REAL to LINT
*
*********************)
FUNCTION TO_LINT__REAL : LINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_LINT__REAL := REAL_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to LINT
*
*********************)
FUNCTION TO_LINT__BOOL : LINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_LINT__BOOL := BOOL_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to LINT
*
*********************)
FUNCTION TO_LINT__BYTE : LINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_LINT__BYTE := BYTE_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts WORD to LINT
*
*********************)
FUNCTION TO_LINT__WORD : LINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_LINT__WORD := WORD_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to LINT
*
*********************)
FUNCTION TO_LINT__DWORD : LINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_LINT__DWORD := DWORD_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to LINT
*
*********************)
FUNCTION TO_LINT__LWORD : LINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_LINT__LWORD := LWORD_TO_LINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to ULINT
*
*********************)
FUNCTION TO_ULINT<T: ANY_NUM> : ULINT
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to ULINT
*
*********************)
FUNCTION TO_ULINT__SINT : ULINT
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_ULINT__SINT := SINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts USINT to ULINT
*
*********************)
FUNCTION TO_ULINT__USINT : ULINT
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_ULINT__USINT := USINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts INT to ULINT
*
*********************)
FUNCTION TO_ULINT__INT : ULINT
    VAR_INPUT
        in : INT;
    END_VAR

    TO_ULINT__INT := INT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts UINT to ULINT
*
*********************)
FUNCTION TO_ULINT__UINT : ULINT
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_ULINT__UINT := UINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts DINT to ULINT
*
*********************)
FUNCTION TO_ULINT__DINT : ULINT
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_ULINT__DINT := DINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts UDINT to ULINT
*
*********************)
FUNCTION TO_ULINT__UDINT : ULINT
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_ULINT__UDINT := UDINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts LINT to ULINT
*
*********************)
FUNCTION TO_ULINT__LINT : ULINT
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_ULINT__LINT := LINT_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts ULINT to ULINT
*
*********************)
FUNCTION TO_ULINT__ULINT : ULINT
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_ULINT__ULINT := in;
END_FUNCTION

(********************
*
* Converts LREAL to ULINT
*
*********************)
FUNCTION TO_ULINT__LREAL : ULINT
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_ULINT__LREAL := LREAL_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts REAL to ULINT
*
*********************)
FUNCTION TO_ULINT__REAL : ULINT
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_ULINT__REAL := REAL_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts BOOL to ULINT
*
*********************)
FUNCTION TO_ULINT__BOOL : ULINT
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_ULINT__BOOL := BOOL_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts BYTE to ULINT
*
*********************)
FUNCTION TO_ULINT__BYTE : ULINT
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_ULINT__BYTE := BYTE_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts WORD to ULINT
*
*********************)
FUNCTION TO_ULINT__WORD : ULINT
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_ULINT__WORD := WORD_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts DWORD to ULINT
*
*********************)
FUNCTION TO_ULINT__DWORD : ULINT
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_ULINT__DWORD := DWORD_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts LWORD to ULINT
*
*********************)
FUNCTION TO_ULINT__LWORD : ULINT
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_ULINT__LWORD := LWORD_TO_ULINT(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to LREAL
*
*********************)
FUNCTION TO_LREAL<T: ANY_NUM> : LREAL
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to LREAL
*
*********************)
FUNCTION TO_LREAL__SINT : LREAL
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_LREAL__SINT := SINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts USINT to LREAL
*
*********************)
FUNCTION TO_LREAL__USINT : LREAL
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_LREAL__USINT := USINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts INT to LREAL
*
*********************)
FUNCTION TO_LREAL__INT : LREAL
    VAR_INPUT
        in : INT;
    END_VAR

    TO_LREAL__INT := INT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts UINT to LREAL
*
*********************)
FUNCTION TO_LREAL__UINT : LREAL
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_LREAL__UINT := UINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts DINT to LREAL
*
*********************)
FUNCTION TO_LREAL__DINT : LREAL
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_LREAL__DINT := DINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts UDINT to LREAL
*
*********************)
FUNCTION TO_LREAL__UDINT : LREAL
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_LREAL__UDINT := UDINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts LINT to LREAL
*
*********************)
FUNCTION TO_LREAL__LINT : LREAL
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_LREAL__LINT := LINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts ULINT to LREAL
*
*********************)
FUNCTION TO_LREAL__ULINT : LREAL
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_LREAL__ULINT := ULINT_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts LREAL to LREAL
*
*********************)
FUNCTION TO_LREAL__LREAL : LREAL
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_LREAL__LREAL := in;
END_FUNCTION

(********************
*
* Converts REAL to LREAL
*
*********************)
FUNCTION TO_LREAL__REAL : LREAL
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_LREAL__REAL := REAL_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts BOOL to LREAL
*
*********************)
FUNCTION TO_LREAL__BOOL : LREAL
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_LREAL__BOOL := BOOL_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts BYTE to LREAL
*
*********************)
FUNCTION TO_LREAL__BYTE : LREAL
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_LREAL__BYTE := BYTE_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts WORD to LREAL
*
*********************)
FUNCTION TO_LREAL__WORD : LREAL
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_LREAL__WORD := WORD_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts DWORD to LREAL
*
*********************)
FUNCTION TO_LREAL__DWORD : LREAL
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_LREAL__DWORD := DWORD_TO_LREAL(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to REAL
*
*********************)
FUNCTION TO_REAL<T: ANY_NUM> : REAL
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to REAL
*
*********************)
FUNCTION TO_REAL__SINT : REAL
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_REAL__SINT := SINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts USINT to REAL
*
*********************)
FUNCTION TO_REAL__USINT : REAL
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_REAL__USINT := USINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts INT to REAL
*
*********************)
FUNCTION TO_REAL__INT : REAL
    VAR_INPUT
        in : INT;
    END_VAR

    TO_REAL__INT := INT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts UINT to REAL
*
*********************)
FUNCTION TO_REAL__UINT : REAL
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_REAL__UINT := UINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts DINT to REAL
*
*********************)
FUNCTION TO_REAL__DINT : REAL
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_REAL__DINT := DINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts UDINT to REAL
*
*********************)
FUNCTION TO_REAL__UDINT : REAL
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_REAL__UDINT := UDINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts LINT to REAL
*
*********************)
FUNCTION TO_REAL__LINT : REAL
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_REAL__LINT := LINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts ULINT to REAL
*
*********************)
FUNCTION TO_REAL__ULINT : REAL
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_REAL__ULINT := ULINT_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts LREAL to REAL
*
*********************)
FUNCTION TO_REAL__LREAL : REAL
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_REAL__LREAL := LREAL_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts REAL to REAL
*
*********************)
FUNCTION TO_REAL__REAL : REAL
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_REAL__REAL := in;
END_FUNCTION

(********************
*
* Converts BOOL to REAL
*
*********************)
FUNCTION TO_REAL__BOOL : REAL
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_REAL__BOOL := BOOL_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts BYTE to REAL
*
*********************)
FUNCTION TO_REAL__BYTE : REAL
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_REAL__BYTE := BYTE_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts WORD to REAL
*
*********************)
FUNCTION TO_REAL__WORD : REAL
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_REAL__WORD := WORD_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts DWORD to REAL
*
*********************)
FUNCTION TO_REAL__DWORD : REAL
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_REAL__DWORD := DWORD_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts LWORD to REAL
*
*********************)
FUNCTION TO_REAL__LWORD : REAL
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_REAL__LWORD := LWORD_TO_REAL(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to BOOL
*
*********************)
FUNCTION TO_BOOL<T: ANY_NUM> : BOOL
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to BOOL
*
*********************)
FUNCTION TO_BOOL__SINT : BOOL
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_BOOL__SINT := SINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts USINT to BOOL
*
*********************)
FUNCTION TO_BOOL__USINT : BOOL
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_BOOL__USINT := USINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts INT to BOOL
*
*********************)
FUNCTION TO_BOOL__INT : BOOL
    VAR_INPUT
        in : INT;
    END_VAR

    TO_BOOL__INT := INT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts UINT to BOOL
*
*********************)
FUNCTION TO_BOOL__UINT : BOOL
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_BOOL__UINT := UINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts DINT to BOOL
*
*********************)
FUNCTION TO_BOOL__DINT : BOOL
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_BOOL__DINT := DINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts UDINT to BOOL
*
*********************)
FUNCTION TO_BOOL__UDINT : BOOL
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_BOOL__UDINT := UDINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts LINT to BOOL
*
*********************)
FUNCTION TO_BOOL__LINT : BOOL
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_BOOL__LINT := LINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts ULINT to BOOL
*
*********************)
FUNCTION TO_BOOL__ULINT : BOOL
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_BOOL__ULINT := ULINT_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts LREAL to BOOL
*
*********************)
FUNCTION TO_BOOL__LREAL : BOOL
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_BOOL__LREAL := LREAL_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts REAL to BOOL
*
*********************)
FUNCTION TO_BOOL__REAL : BOOL
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_BOOL__REAL := REAL_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts BOOL to BOOL
*
*********************)
FUNCTION TO_BOOL__BOOL : BOOL
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_BOOL__BOOL := in;
END_FUNCTION

(********************
*
* Converts BYTE to BOOL
*
*********************)
FUNCTION TO_BOOL__BYTE : BOOL
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_BOOL__BYTE := BYTE_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts WORD to BOOL
*
*********************)
FUNCTION TO_BOOL__WORD : BOOL
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_BOOL__WORD := WORD_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts DWORD to BOOL
*
*********************)
FUNCTION TO_BOOL__DWORD : BOOL
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_BOOL__DWORD := DWORD_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts LWORD to BOOL
*
*********************)
FUNCTION TO_BOOL__LWORD : BOOL
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_BOOL__LWORD := LWORD_TO_BOOL(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to BYTE
*
*********************)
FUNCTION TO_BYTE<T: ANY_NUM> : BYTE
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to BYTE
*
*********************)
FUNCTION TO_BYTE__SINT : BYTE
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_BYTE__SINT := SINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts USINT to BYTE
*
*********************)
FUNCTION TO_BYTE__USINT : BYTE
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_BYTE__USINT := USINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts INT to BYTE
*
*********************)
FUNCTION TO_BYTE__INT : BYTE
    VAR_INPUT
        in : INT;
    END_VAR

    TO_BYTE__INT := INT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts UINT to BYTE
*
*********************)
FUNCTION TO_BYTE__UINT : BYTE
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_BYTE__UINT := UINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts DINT to BYTE
*
*********************)
FUNCTION TO_BYTE__DINT : BYTE
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_BYTE__DINT := DINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts UDINT to BYTE
*
*********************)
FUNCTION TO_BYTE__UDINT : BYTE
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_BYTE__UDINT := UDINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts LINT to BYTE
*
*********************)
FUNCTION TO_BYTE__LINT : BYTE
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_BYTE__LINT := LINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts ULINT to BYTE
*
*********************)
FUNCTION TO_BYTE__ULINT : BYTE
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_BYTE__ULINT := ULINT_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts LREAL to BYTE
*
*********************)
FUNCTION TO_BYTE__LREAL : BYTE
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_BYTE__LREAL := LREAL_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts REAL to BYTE
*
*********************)
FUNCTION TO_BYTE__REAL : BYTE
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_BYTE__REAL := REAL_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts BOOL to BYTE
*
*********************)
FUNCTION TO_BYTE__BOOL : BYTE
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_BYTE__BOOL := BOOL_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts BYTE to BYTE
*
*********************)
FUNCTION TO_BYTE__BYTE : BYTE
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_BYTE__BYTE := in;
END_FUNCTION

(********************
*
* Converts WORD to BYTE
*
*********************)
FUNCTION TO_BYTE__WORD : BYTE
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_BYTE__WORD := WORD_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts DWORD to BYTE
*
*********************)
FUNCTION TO_BYTE__DWORD : BYTE
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_BYTE__DWORD := DWORD_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts LWORD to BYTE
*
*********************)
FUNCTION TO_BYTE__LWORD : BYTE
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_BYTE__LWORD := LWORD_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to WORD
*
*********************)
FUNCTION TO_WORD<T: ANY_NUM> : WORD
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to WORD
*
*********************)
FUNCTION TO_WORD__SINT : WORD
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_WORD__SINT := SINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts USINT to WORD
*
*********************)
FUNCTION TO_WORD__USINT : WORD
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_WORD__USINT := USINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts INT to WORD
*
*********************)
FUNCTION TO_WORD__INT : WORD
    VAR_INPUT
        in : INT;
    END_VAR

    TO_WORD__INT := INT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts UINT to WORD
*
*********************)
FUNCTION TO_WORD__UINT : WORD
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_WORD__UINT := UINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts DINT to WORD
*
*********************)
FUNCTION TO_WORD__DINT : WORD
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_WORD__DINT := DINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts UDINT to WORD
*
*********************)
FUNCTION TO_WORD__UDINT : WORD
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_WORD__UDINT := UDINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts LINT to WORD
*
*********************)
FUNCTION TO_WORD__LINT : WORD
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_WORD__LINT := LINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts ULINT to WORD
*
*********************)
FUNCTION TO_WORD__ULINT : WORD
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_WORD__ULINT := ULINT_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts LREAL to WORD
*
*********************)
FUNCTION TO_WORD__LREAL : WORD
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_WORD__LREAL := LREAL_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts REAL to WORD
*
*********************)
FUNCTION TO_WORD__REAL : WORD
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_WORD__REAL := REAL_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts BOOL to WORD
*
*********************)
FUNCTION TO_WORD__BOOL : WORD
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_WORD__BOOL := BOOL_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts BYTE to WORD
*
*********************)
FUNCTION TO_WORD__BYTE : WORD
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_WORD__BYTE := BYTE_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts WORD to WORD
*
*********************)
FUNCTION TO_WORD__WORD : WORD
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_WORD__WORD := in;
END_FUNCTION

(********************
*
* Converts DWORD to WORD
*
*********************)
FUNCTION TO_WORD__DWORD : WORD
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_WORD__DWORD := DWORD_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts LWORD to WORD
*
*********************)
FUNCTION TO_WORD__LWORD : WORD
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_WORD__LWORD := LWORD_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to DWORD
*
*********************)
FUNCTION TO_DWORD<T: ANY_NUM> : DWORD
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to DWORD
*
*********************)
FUNCTION TO_DWORD__SINT : DWORD
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_DWORD__SINT := SINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts USINT to DWORD
*
*********************)
FUNCTION TO_DWORD__USINT : DWORD
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_DWORD__USINT := USINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts INT to DWORD
*
*********************)
FUNCTION TO_DWORD__INT : DWORD
    VAR_INPUT
        in : INT;
    END_VAR

    TO_DWORD__INT := INT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts UINT to DWORD
*
*********************)
FUNCTION TO_DWORD__UINT : DWORD
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_DWORD__UINT := UINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts DINT to DWORD
*
*********************)
FUNCTION TO_DWORD__DINT : DWORD
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_DWORD__DINT := DINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts UDINT to DWORD
*
*********************)
FUNCTION TO_DWORD__UDINT : DWORD
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_DWORD__UDINT := UDINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts LINT to DWORD
*
*********************)
FUNCTION TO_DWORD__LINT : DWORD
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_DWORD__LINT := LINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts ULINT to DWORD
*
*********************)
FUNCTION TO_DWORD__ULINT : DWORD
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_DWORD__ULINT := ULINT_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts LREAL to DWORD
*
*********************)
FUNCTION TO_DWORD__LREAL : DWORD
    VAR_INPUT
        in : LREAL;
    END_VAR

    VAR_TEMP
        tmp : REAL;
    END_VAR

    tmp := in;

    TO_DWORD__LREAL := REAL_TO_DWORD(tmp);
END_FUNCTION

(********************
*
* Converts REAL to DWORD
*
*********************)
FUNCTION TO_DWORD__REAL : DWORD
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_DWORD__REAL := REAL_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts BOOL to DWORD
*
*********************)
FUNCTION TO_DWORD__BOOL : DWORD
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_DWORD__BOOL := BOOL_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts BYTE to DWORD
*
*********************)
FUNCTION TO_DWORD__BYTE : DWORD
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_DWORD__BYTE := BYTE_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts WORD to DWORD
*
*********************)
FUNCTION TO_DWORD__WORD : DWORD
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_DWORD__WORD := WORD_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts DWORD to DWORD
*
*********************)
FUNCTION TO_DWORD__DWORD : DWORD
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_DWORD__DWORD := in;
END_FUNCTION

(********************
*
* Converts LWORD to DWORD
*
*********************)
FUNCTION TO_DWORD__LWORD : DWORD
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_DWORD__LWORD := LWORD_TO_DWORD(in);
END_FUNCTION

(********************
*
* Converts any other numerical value to LWORD
*
*********************)
FUNCTION TO_LWORD<T: ANY_NUM> : LWORD
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

(********************
*
* Converts SINT to LWORD
*
*********************)
FUNCTION TO_LWORD__SINT : LWORD
    VAR_INPUT
        in : SINT;
    END_VAR

    TO_LWORD__SINT := SINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts USINT to LWORD
*
*********************)
FUNCTION TO_LWORD__USINT : LWORD
    VAR_INPUT
        in : USINT;
    END_VAR

    TO_LWORD__USINT := USINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts INT to LWORD
*
*********************)
FUNCTION TO_LWORD__INT : LWORD
    VAR_INPUT
        in : INT;
    END_VAR

    TO_LWORD__INT := INT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts UINT to LWORD
*
*********************)
FUNCTION TO_LWORD__UINT : LWORD
    VAR_INPUT
        in : UINT;
    END_VAR

    TO_LWORD__UINT := UINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts DINT to LWORD
*
*********************)
FUNCTION TO_LWORD__DINT : LWORD
    VAR_INPUT
        in : DINT;
    END_VAR

    TO_LWORD__DINT := DINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts UDINT to LWORD
*
*********************)
FUNCTION TO_LWORD__UDINT : LWORD
    VAR_INPUT
        in : UDINT;
    END_VAR

    TO_LWORD__UDINT := UDINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts LINT to LWORD
*
*********************)
FUNCTION TO_LWORD__LINT : LWORD
    VAR_INPUT
        in : LINT;
    END_VAR

    TO_LWORD__LINT := LINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts ULINT to LWORD
*
*********************)
FUNCTION TO_LWORD__ULINT : LWORD
    VAR_INPUT
        in : ULINT;
    END_VAR

    TO_LWORD__ULINT := ULINT_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts LREAL to LWORD
*
*********************)
FUNCTION TO_LWORD__LREAL : LWORD
    VAR_INPUT
        in : LREAL;
    END_VAR

    TO_LWORD__LREAL := LREAL_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts REAL to LWORD
*
*********************)
FUNCTION TO_LWORD__REAL : LWORD
    VAR_INPUT
        in : REAL;
    END_VAR

    TO_LWORD__REAL := REAL_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts BOOL to LWORD
*
*********************)
FUNCTION TO_LWORD__BOOL : LWORD
    VAR_INPUT
        in : BOOL;
    END_VAR

    TO_LWORD__BOOL := BOOL_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts BYTE to LWORD
*
*********************)
FUNCTION TO_LWORD__BYTE : LWORD
    VAR_INPUT
        in : BYTE;
    END_VAR

    TO_LWORD__BYTE := BYTE_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts WORD to LWORD
*
*********************)
FUNCTION TO_LWORD__WORD : LWORD
    VAR_INPUT
        in : WORD;
    END_VAR

    TO_LWORD__WORD := WORD_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts DWORD to LWORD
*
*********************)
FUNCTION TO_LWORD__DWORD : LWORD
    VAR_INPUT
        in : DWORD;
    END_VAR

    TO_LWORD__DWORD := DWORD_TO_LWORD(in);
END_FUNCTION

(********************
*
* Converts LWORD to LWORD
*
*********************)
FUNCTION TO_LWORD__LWORD : LWORD
    VAR_INPUT
        in : LWORD;
    END_VAR

    TO_LWORD__LWORD := in;
END_FUNCTION



// (********************
// *
// * Converts any other numerical value to STRING
// *
// *********************)
// FUNCTION TO_STRING<T: ANY_NUM> : STRING
//     VAR_INPUT
//         in : T;
//     END_VAR
// END_FUNCTION

// (********************
// *
// * Converts SINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__SINT : STRING
//     VAR_INPUT
//         in : SINT;
//     END_VAR

//     TO_STRING__SINT := SINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts USINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__USINT : STRING
//     VAR_INPUT
//         in : USINT;
//     END_VAR

//     TO_STRING__USINT := USINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts INT to STRING
// *
// *********************)
// FUNCTION TO_STRING__INT : STRING
//     VAR_INPUT
//         in : INT;
//     END_VAR

//     TO_STRING__INT := INT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts UINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__UINT : STRING
//     VAR_INPUT
//         in : UINT;
//     END_VAR

//     TO_STRING__UINT := UINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts DINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__DINT : STRING
//     VAR_INPUT
//         in : DINT;
//     END_VAR

//     TO_STRING__DINT := DINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts UDINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__UDINT : STRING
//     VAR_INPUT
//         in : UDINT;
//     END_VAR

//     TO_STRING__UDINT := UDINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__LINT : STRING
//     VAR_INPUT
//         in : LINT;
//     END_VAR

//     TO_STRING__LINT := LINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts ULINT to STRING
// *
// *********************)
// FUNCTION TO_STRING__ULINT : STRING
//     VAR_INPUT
//         in : ULINT;
//     END_VAR

//     TO_STRING__ULINT := ULINT_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LREAL to STRING
// *
// *********************)
// FUNCTION TO_STRING__LREAL : STRING
//     VAR_INPUT
//         in : LREAL;
//     END_VAR

//     TO_STRING__LREAL := LREAL_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts REAL to STRING
// *
// *********************)
// FUNCTION TO_STRING__REAL : STRING
//     VAR_INPUT
//         in : REAL;
//     END_VAR

//     TO_STRING__REAL := REAL_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts BOOL to STRING
// *
// *********************)
// FUNCTION TO_STRING__BOOL : STRING
//     VAR_INPUT
//         in : BOOL;
//     END_VAR

//     TO_STRING__BOOL := BOOL_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts BYTE to STRING
// *
// *********************)
// FUNCTION TO_STRING__BYTE : STRING
//     VAR_INPUT
//         in : BYTE;
//     END_VAR

//     TO_STRING__BYTE := BYTE_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts WORD to STRING
// *
// *********************)
// FUNCTION TO_STRING__WORD : STRING
//     VAR_INPUT
//         in : WORD;
//     END_VAR

//     TO_STRING__WORD := WORD_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts DWORD to STRING
// *
// *********************)
// FUNCTION TO_STRING__DWORD : STRING
//     VAR_INPUT
//         in : DWORD;
//     END_VAR

//     TO_STRING__DWORD := DWORD_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LWORD to STRING
// *
// *********************)
// FUNCTION TO_STRING__LWORD : STRING
//     VAR_INPUT
//         in : LWORD;
//     END_VAR

//     TO_STRING__LWORD := LWORD_TO_STRING(in);
// END_FUNCTION

// (********************
// *
// * Converts any other numerical value to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING<T: ANY_NUM> : WSTRING
//     VAR_INPUT
//         in : T;
//     END_VAR
// END_FUNCTION

// (********************
// *
// * Converts SINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__SINT : WSTRING
//     VAR_INPUT
//         in : SINT;
//     END_VAR

//     TO_WSTRING__SINT := SINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts USINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__USINT : WSTRING
//     VAR_INPUT
//         in : USINT;
//     END_VAR

//     TO_WSTRING__USINT := USINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts INT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__INT : WSTRING
//     VAR_INPUT
//         in : INT;
//     END_VAR

//     TO_WSTRING__INT := INT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts UINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__UINT : WSTRING
//     VAR_INPUT
//         in : UINT;
//     END_VAR

//     TO_WSTRING__UINT := UINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts DINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__DINT : WSTRING
//     VAR_INPUT
//         in : DINT;
//     END_VAR

//     TO_WSTRING__DINT := DINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts UDINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__UDINT : WSTRING
//     VAR_INPUT
//         in : UDINT;
//     END_VAR

//     TO_WSTRING__UDINT := UDINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__LINT : WSTRING
//     VAR_INPUT
//         in : LINT;
//     END_VAR

//     TO_WSTRING__LINT := LINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts ULINT to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__ULINT : WSTRING
//     VAR_INPUT
//         in : ULINT;
//     END_VAR

//     TO_WSTRING__ULINT := ULINT_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LREAL to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__LREAL : WSTRING
//     VAR_INPUT
//         in : LREAL;
//     END_VAR

//     TO_WSTRING__LREAL := LREAL_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts REAL to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__REAL : WSTRING
//     VAR_INPUT
//         in : REAL;
//     END_VAR

//     TO_WSTRING__REAL := REAL_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts BOOL to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__BOOL : WSTRING
//     VAR_INPUT
//         in : BOOL;
//     END_VAR

//     TO_WSTRING__BOOL := BOOL_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts BYTE to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__BYTE : WSTRING
//     VAR_INPUT
//         in : BYTE;
//     END_VAR

//     TO_WSTRING__BYTE := BYTE_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts WORD to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__WORD : WSTRING
//     VAR_INPUT
//         in : WORD;
//     END_VAR

//     TO_WSTRING__WORD := WORD_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts DWORD to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__DWORD : WSTRING
//     VAR_INPUT
//         in : DWORD;
//     END_VAR

//     TO_WSTRING__DWORD := DWORD_TO_WSTRING(in);
// END_FUNCTION

// (********************
// *
// * Converts LWORD to WSTRING
// *
// *********************)
// FUNCTION TO_WSTRING__LWORD : WSTRING
//     VAR_INPUT
//         in : LWORD;
//     END_VAR

//     TO_WSTRING__LWORD := LWORD_TO_WSTRING(in);
// END_FUNCTION

