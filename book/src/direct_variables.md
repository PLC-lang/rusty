# Direct (Bit) Access on Variables

The IEC61131-3 Standard allows reading specific `Bits`, `Bytes`, `Words` or `DWords` from an `ANY_BIT` type.
RuSTy supports this functionalty and extends it to support all `INT` types.

## Constant based Direct Access
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

## Varirable based Direct Access

While the IEC61131-3 Standard only defines variable access using constant int literals, 
RuSTy additionally supports access using Variables
The Syntax for a variable based access is `%<Type><Variable>`
The provided varibale has to be a direct Reference variable (non Qualified)

> _Short hand access for Bit (Without the `%X` modifier) is not allowed._

### Example 
```st
FUNCTION main : DINT
VAR 
    variable    : LWORD; 
    access_var  : INT;
    bitTarget   : BOOL;
    bitTarget2  : BOOL;
    byteTarget  : BYTE;
    wordTarget  : WORD;
    dwordTarget : DWORD;
END_VAR
variable    := 16#AB_CD_EF_12_34_56_78_90;
access_var := 63;
bitTarget   := variable.%Xaccess_var; (*Access last bit*)
access_var := 7;
byteTarget  := variable.%Baccess_var; (*Access last byte*)
access_var := 3;
wordTarget  := variable.%Waccess_var; (*Access last word*)
access_var := 1;
dwordTarget := variable.%Daccess_var; (*Access last dword*)
(*Chaining an access is also allowed *)
bitTarget2  := variable.%Daccess_var.%Waccess_var.%Baccess_var.%Xaccess_var;
END_FUNCTION
```
