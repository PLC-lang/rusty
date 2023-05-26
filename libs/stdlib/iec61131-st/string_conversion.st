(********************
*
* Converts WSTRING to STRING
*
*********************)
FUNCTION WSTRING_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	in : WSTRING[__STRING_LENGTH];
END_VAR
	WSTRING_TO_STRING_EXT(in, WSTRING_TO_STRING);
END_FUNCTION

{external}
FUNCTION WSTRING_TO_STRING_EXT : DINT
VAR_INPUT {ref}
	in : WSTRING[__STRING_LENGTH];
END_VAR
VAR_IN_OUT 
	out : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(********************
*
* Converts WSTRING to WCHAR
*
*********************)
FUNCTION WSTRING_TO_WCHAR : WCHAR
VAR_INPUT {ref}
	in : WSTRING;
END_VAR
VAR
	ptr : REF_TO WCHAR;
END_VAR
	ptr := &in;
	WSTRING_TO_WCHAR := ptr^;
END_FUNCTION

(********************
*
* Converts STRING to WSTRING
* Will return a WSTRING with default length of 81
*
*********************)
FUNCTION STRING_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
	in : STRING[__STRING_LENGTH];
END_VAR
	STRING_TO_WSTRING_EXT(in, STRING_TO_WSTRING);
END_FUNCTION

{external}
FUNCTION STRING_TO_WSTRING_EXT : DINT
VAR_INPUT {ref}
	in : STRING[__STRING_LENGTH];
END_VAR
VAR_IN_OUT
	out : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(********************
*
* Converts STRING to CHAR
*
*********************)
FUNCTION STRING_TO_CHAR : CHAR
VAR_INPUT {ref}
	in : STRING;
END_VAR
VAR
	ptr : REF_TO CHAR;
END_VAR
	ptr := &in;
	STRING_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to WSTRING
*
*********************)
FUNCTION WCHAR_TO_WSTRING : WSTRING
VAR_INPUT
	in : WCHAR;
END_VAR
VAR
	ptr : REF_TO WSTRING;
END_VAR
	ptr := &in;
	WCHAR_TO_WSTRING := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to CHAR
*
*********************)
{external}
FUNCTION WCHAR_TO_CHAR : CHAR
VAR_INPUT
	in : WCHAR;
END_VAR
END_FUNCTION

(********************
*
* Converts CHAR to STRING
*
*********************)
FUNCTION CHAR_TO_STRING : STRING
VAR_INPUT
	in : CHAR;
END_VAR
VAR
	ptr : REF_TO STRING;
END_VAR
	ptr := &in;
	CHAR_TO_STRING := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to WCHAR
*
*********************)
{external}
FUNCTION CHAR_TO_WCHAR : WCHAR
VAR_INPUT
	in : CHAR;
END_VAR
END_FUNCTION