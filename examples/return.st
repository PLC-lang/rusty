FUNCTION identity : WSTRING
VAR_INPUT
    in: WSTRING;
END_VAR
    foo := in;
END_FUNCTION

FUNCTION main : DINT
VAR
    s: STRING;
END_VAR
    s := identity("wstring"); // cannot assign wstring to string
END_FUNCTION

// lowered:
FUNCTION identity
VAR_OUTPUT
    return: STRING;
END_VAR
VAR_INPUT
    in: STRING;
END_VAR
    return := in;
END_FUNCTION

FUNCTION main : DINT
VAR
    s: STRING;
END_VAR
    identity(s, "wstring"); // cannot assign string to wstring
END_FUNCTION
