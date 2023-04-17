(********************
*
* Converts little endian data format to big endian data format
*
*********************)
{external}
FUNCTION TO_BIG_ENDIAN<T: ANY> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts big endian data format to little endian data format
*
*********************)
{external}
FUNCTION TO_LITTLE_ENDIAN<T: ANY> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to little endian data format from big endian data format
*
*********************)
{external}
FUNCTION FROM_BIG_ENDIAN<T: ANY> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to big endian data format from little endian data format
*
*********************)
{external}
FUNCTION FROM_LITTLE_ENDIAN<T: ANY> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

