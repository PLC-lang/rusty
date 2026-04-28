{external}
FUNCTION puts: DINT
VAR_INPUT {ref}
    text: STRING;
END_VAR
END_FUNCTION

FUNCTION main: DINT
    puts('Hello World!$N');
    main := 0;
END_FUNCTION
