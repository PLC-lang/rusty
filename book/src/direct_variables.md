# Direct (Bit) Access on Variables

The IEC61131 Standard allows reading specific `Bits`, `Bytes`, `Words` or `DWords` from an `ANY_BIT` type.
RuSTy supports this functionalty and extends it to support all `INT` types.

## Usage
To access a bit sequence in a variable, a direct access instruction `%<Type><Value>` is used.
`Type` is the bit sequence size required and is described as follows : 

| Type | Size | Example |
|----- |------|---------|
| X    | 1    | `%X1    |  
| B    | 8    | `%B1    |  
| W    | 16   | `%W1    |  
| D    | 32   | `%D1    |  

> _For `Bit` access, the `%X` is optional._

### Example 
```st
FUNCTION main : DINT
VAR 
    variable    : LWORD; 
    bitTarget   : BOOL;
    bitTarget2  : BOOL;
    byteTarget  : BYTE;
    wordTarget  : WORD;
    dwordTarget : DWORD;
END_VAR

variable    := 16#AB_CD_EF_12_34_56_78_90;
bitTarget   := variable.%X63; (*Access last bit*)
byteTarget  := variable.%B7; (*Access last byte*)
wordTarget  := variable.%W3; (*Access last word*)
dwordTarget := variable.%D1; (*Access last dword*)
(*Chaining an access is also allowed *)
bitTarget2  := variable.%D1.%W1.%B1.%X1;

END_FUNCTION
```
