VAR_GLOBAL CONSTANT
    __STRING_LENGTH : DINT := 2048;
END_VAR

(******************************************************************************
Description: String character length
Input: 
    - IN:   A character string
Return: String length
******************************************************************************)
{external}
FUNCTION LEN <T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Left
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the leftmost L characters of IN
******************************************************************************)
FUNCTION LEFT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION LEFT_EXT<T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION LEFT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__STRING);
END_FUNCTION

FUNCTION LEFT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__WSTRING);
END_FUNCTION

{external}
FUNCTION LEFT_EXT__STRING : DINT
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION LEFT_EXT__WSTRING : DINT
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Right
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the rightmost L characters of IN
******************************************************************************)
FUNCTION RIGHT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION RIGHT_EXT<T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION RIGHT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__STRING);
END_FUNCTION

FUNCTION RIGHT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__WSTRING);
END_FUNCTION

{external}
FUNCTION RIGHT_EXT__STRING : DINT
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION RIGHT_EXT__WSTRING : DINT
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT: WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Middle
Input:
    - IN:   A character string
    - L:    The length of the substring
    - P:    The starting index position
Return: 
    A substring that contains L characters starting 
    from position P in a string.
******************************************************************************)
FUNCTION MID <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION MID_EXT<T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION MID__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, P, MID__STRING);
END_FUNCTION

FUNCTION MID__WSTRING : WSTRING[__STRING_LENGTH] 
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, P, MID__WSTRING);
END_FUNCTION

{external}
FUNCTION MID_EXT__STRING : DINT
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION MID_EXT__WSTRING : DINT
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION


(******************************************************************************
Description: Extensible concatenation
Input:
    - IN:   Two or more comma-separated strings
Return:
    A string combining all given input strings in the same order
    as the given string parameters.
******************************************************************************)
{external}
FUNCTION CONCAT__STRING : STRING[2048]
VAR_INPUT {ref}
	args : {sized} STRING...;
END_VAR
END_FUNCTION

{external}
FUNCTION CONCAT__WSTRING : WSTRING[2048]
VAR_INPUT {ref}
	args : {sized} WSTRING...;
END_VAR
END_FUNCTION

FUNCTION CONCAT <T: ANY_STRING> : T
VAR_INPUT {ref}
	args : {sized} T...;
END_VAR
END_FUNCTION

{external}
FUNCTION CONCAT_EXT<T: ANY_STRING> : DINT
VAR_IN_OUT
    OUT: T;
END_VAR
VAR_INPUT {ref}
    args: {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Insert
Input:
    IN1:    The string to insert into
    IN2:    The string to insert
    P:      The position at which to insert
Return:
    A string consisting of IN2 inserted at position P into string IN1
******************************************************************************)
FUNCTION INSERT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION INSERT_EXT<T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION INSERT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT__STRING);
END_FUNCTION

FUNCTION INSERT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT__WSTRING);
END_FUNCTION

{external}
FUNCTION INSERT_EXT__STRING : DINT
VAR_INPUT {ref}
	IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT: STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION INSERT_EXT__WSTRING : DINT
VAR_INPUT {ref}
	IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT: WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Delete
Input: 
    IN: The string to delete characters from
    L:  The amount of characters to delete
    P:  The position at which to start deleting
Return: A new string with L characters deleted from P onwards
******************************************************************************)
FUNCTION DELETE <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION DELETE_EXT<T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION DELETE__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE__STRING);
END_FUNCTION

FUNCTION DELETE__WSTRING : WSTRING[__STRING_LENGTH] 
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE__WSTRING);
END_FUNCTION

{external}
FUNCTION DELETE_EXT__STRING : DINT
VAR_INPUT {ref}
	IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION DELETE_EXT__WSTRING : DINT
VAR_INPUT {ref}
	IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT: WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION
(******************************************************************************
Description: Replace
Input:
    IN1:    The string to replace characters from
    IN2:    The replacement string to be inserted
    L:      The amount of characters to delete
    P:      The position at which to start deleting
Return: A new string which has L characters replaced by IN2 from position P onwards
******************************************************************************)
FUNCTION REPLACE <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION REPLACE_EXT <T: ANY_STRING> : DINT
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT 
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR    
END_FUNCTION

FUNCTION REPLACE__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
	IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE__STRING);
END_FUNCTION

FUNCTION REPLACE__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR    
VAR_INPUT 
    L   : DINT;
    P   : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE__WSTRING);
END_FUNCTION

{external}
FUNCTION REPLACE_EXT__STRING : DINT
VAR_INPUT {ref}
    IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT 
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR    
END_FUNCTION

{external}
FUNCTION REPLACE_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT 
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR    
END_FUNCTION


(******************************************************************************
Description: Find
Input: 
    IN1:    The string in which to search in
    IN2:    The substring to search for
Return: The character index of the first match. 0 if there are no matches.
******************************************************************************)
{external}
FUNCTION FIND <T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Decreasing sequence
Input:  The strings to compare, in order. ((IN1>IN2) & (IN2>IN3) & .. & (INn-1>INn))
Return: 
    TRUE if codepoints are in decreasing order (alphabetical order: ZYX > YXW > XWV ..)
    FALSE otherwise
******************************************************************************)
{external}
FUNCTION GT <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : {sized} T...;
END_VAR
END_FUNCTION

// passing strings by ref causes an error/warning. code still compiles and works correctly
FUNCTION STRING_GREATER : BOOL
VAR_INPUT 
    a, b : STRING;
END_VAR
    STRING_GREATER := GT(a, b);
END_FUNCTION

FUNCTION WSTRING_GREATER : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_GREATER := GT(a, b);
END_FUNCTION
(******************************************************************************
Description: Monotonic sequence
Input:  The strings to compare, in order. ((IN1>=IN2) & (IN2>=IN3) & .. & (INn-1>=INn))
Return: 
    TRUE if codepoints are in decreasing order or are equal to adjacent codepoints (alphabetical order: ZYX >= ZYX >= YXW ..)
    FALSE otherwise
******************************************************************************)
{external}
FUNCTION GE <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Equality
Input: The strings to compare, in order. ((IN1=IN2) & (IN2=IN3) & .. & (INn-1=INn))
Return: 
    TRUE if codepoints are equal to each other
    FALSE otherwise.
******************************************************************************)
{external}
FUNCTION EQ <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : {sized} T...;
END_VAR
END_FUNCTION

FUNCTION STRING_EQUAL : BOOL
VAR_INPUT
    a, b : STRING;
END_VAR
    STRING_EQUAL := EQ(a, b);
END_FUNCTION

FUNCTION WSTRING_EQUAL : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_EQUAL := EQ(a, b);
END_FUNCTION

(******************************************************************************
Description: Monotonic sequence
Input:  The strings to compare, in order. ((IN1<=IN2) & (IN2<=IN3) & .. & (INn-1<=INn))
Return: 
    TRUE if codepoints are in increasing order or are equal to adjacent codepoints (alphabetical order: ABC <= ABC <= BCD ..)
    FALSE otherwise
******************************************************************************)
{external}
FUNCTION LE <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Increasing sequence
Input:  The strings to compare, in order. ((IN1<IN2) & (IN2<IN3) & .. & (INn-1<INn))
Return: 
    TRUE if codepoints are in increasing order (alphabetical order: ABC < BCD < CDE ..)
    FALSE otherwise
******************************************************************************)
{external}
FUNCTION LT <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : {sized} T...;
END_VAR
END_FUNCTION

FUNCTION STRING_LESS : BOOL
VAR_INPUT
    a, b : STRING;
END_VAR
    STRING_LESS := LT(a, b);
END_FUNCTION

FUNCTION WSTRING_LESS : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_LESS := LT(a, b);
END_FUNCTION

(******************************************************************************
Description: Inequality
Input: The strings to compare. (IN1<>IN2)
Return:
    TRUE if strings do not match
    FALSE otherwise
******************************************************************************)
{external}
FUNCTION NE <T: ANY_STRING> : BOOL
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
END_FUNCTION