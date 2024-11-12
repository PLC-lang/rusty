FUNCTION TO_CHAR<T: ANY> : CHAR
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_CHAR__CHAR : CHAR
    VAR_INPUT
        in : T;
    END_VAR

    TO_CHAR__CHAR := in;
END_FUNCTION

FUNCTION TO_CHAR__WCHAR : CHAR
    VAR_INPUT
        in : WCHAR;
    END_VAR

    TO_CHAR__WCHAR := WCHAR_TO_CHAR(in);
END_FUNCTION

FUNCTION TO_CHAR__STRING : CHAR
    VAR_INPUT {ref}
        in : STRING;
    END_VAR

    TO_CHAR__STRING := STRING_TO_CHAR(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_WCHAR<T: ANY> : WCHAR
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_WCHAR__CHAR : WCHAR
    VAR_INPUT
        in : CHAR;
    END_VAR

    TO_WCHAR__CHAR := CHAR_TO_WCHAR(in);

END_FUNCTION

FUNCTION TO_WCHAR__WSTRING : WCHAR
    VAR_INPUT {ref}
        in : WSTRING;
    END_VAR

    TO_WCHAR__WSTRING := WSTRING_TO_WCHAR(in);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------

FUNCTION TO_STRING<T: ANY> : STRING[__STRING_LENGTH]
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_STRING__CHAR : STRING
    VAR_INPUT
        in : CHAR;
    END_VAR

    TO_STRING__CHAR := CHAR_TO_STRING(in);
END_FUNCTION

FUNCTION TO_STRING__WSTRING : STRING[__STRING_LENGTH]
    VAR_INPUT {ref}
        in : WSTRING;
    END_VAR

    WSTRING_TO_STRING_EXT(in, TO_STRING__WSTRING);
END_FUNCTION

// -----------------------------------------------------------------------------------------------------------
FUNCTION TO_WSTRING<T: ANY> : WSTRING[__STRING_LENGTH]
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

FUNCTION TO_WSTRING__STRING : WSTRING[__STRING_LENGTH]
    VAR_INPUT {ref}
        in : STRING;
    END_VAR

    STRING_TO_WSTRING_EXT(in, TO_WSTRING__STRING);
END_FUNCTION

FUNCTION TO_WSTRING__WCHAR : WSTRING[__STRING_LENGTH]
    VAR_INPUT
        in : WCHAR;
    END_VAR

    TO_WSTRING__WCHAR := WCHAR_TO_WSTRING(in);
END_FUNCTION