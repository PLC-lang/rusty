(********************
*
* Converts LWORD to DWORD
*
*********************)
FUNCTION LWORD_TO_DWORD : DWORD
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts LWORD to WORD
*
*********************)
FUNCTION LWORD_TO_WORD : WORD
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts LWORD to BYTE
*
*********************)
FUNCTION LWORD_TO_BYTE : BYTE
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts LWORD to BOOL
*
*********************)
FUNCTION LWORD_TO_BOOL : BOOL
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts DWORD to LWORD
*
*********************)
FUNCTION DWORD_TO_LWORD : LWORD
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts DWORD to WORD
*
*********************)
FUNCTION DWORD_TO_WORD : WORD
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts DWORD to BYTE
*
*********************)
FUNCTION DWORD_TO_BYTE : BYTE
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts DWORD to BOOL
*
*********************)
FUNCTION DWORD_TO_BOOL : BOOL
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts WORD to LWORD
*
*********************)
FUNCTION WORD_TO_LWORD : LWORD
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts WORD to DWORD
*
*********************)
FUNCTION WORD_TO_DWORD : DWORD
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts WORD to BYTE
*
*********************)
FUNCTION WORD_TO_BYTE : BYTE
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts WORD to BOOL
*
*********************)
FUNCTION WORD_TO_BOOL : BOOL
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts BYTE to LWORD
*
*********************)
FUNCTION BYTE_TO_LWORD : LWORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to DWORD
*
*********************)
FUNCTION BYTE_TO_DWORD : DWORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to WORD
*
*********************)
FUNCTION BYTE_TO_WORD : WORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to BOOL
*
*********************)
FUNCTION BYTE_TO_BOOL : BOOL
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts BYTE to CHAR
*
*********************)
FUNCTION BYTE_TO_CHAR : CHAR
VAR_INPUT
    in : BYTE;
END_VAR
VAR
    ptr : REF_TO CHAR;
END_VAR
    ptr := REF(in);
    BYTE_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts BOOL to LWORD
*
*********************)
FUNCTION BOOL_TO_LWORD : LWORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_LWORD := 1;
    ELSE
        BOOL_TO_LWORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to DWORD
*
*********************)
FUNCTION BOOL_TO_DWORD : DWORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_DWORD := 1;
    ELSE
        BOOL_TO_DWORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to WORD
*
*********************)
FUNCTION BOOL_TO_WORD : WORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_WORD := 1;
    ELSE
        BOOL_TO_WORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to BYTE
*
*********************)
FUNCTION BOOL_TO_BYTE : BYTE
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_BYTE := 1;
    ELSE
        BOOL_TO_BYTE := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts CHAR to BYTE
*
*********************)
FUNCTION CHAR_TO_BYTE : BYTE
VAR_INPUT
    in : CHAR;
END_VAR
VAR
    ptr : REF_TO BYTE;
END_VAR
    ptr := REF(in);
    CHAR_TO_BYTE := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to WORD
*
*********************)
FUNCTION CHAR_TO_WORD : WORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_WORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts CHAR to DWORD
*
*********************)
FUNCTION CHAR_TO_DWORD : DWORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_DWORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts CHAR to LWORD
*
*********************)
FUNCTION CHAR_TO_LWORD : LWORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_LWORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts WCHAR to WORD
*
*********************)
FUNCTION WCHAR_TO_WORD : WORD
VAR_INPUT
    in : WCHAR;
END_VAR
VAR
    ptr : REF_TO WORD;
END_VAR
    ptr := REF(in);
    WCHAR_TO_WORD := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to DWORD
*
*********************)
FUNCTION WCHAR_TO_DWORD : DWORD
VAR_INPUT
    in : WCHAR;
END_VAR
    WCHAR_TO_DWORD := WCHAR_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts WCHAR to LWORD
*
*********************)
FUNCTION WCHAR_TO_LWORD : LWORD
VAR_INPUT
    in : WCHAR;
END_VAR
    WCHAR_TO_LWORD := WCHAR_TO_WORD(in);
END_FUNCTION
