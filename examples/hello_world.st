{external} 
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION main : DINT
	puts('hello, world!$N');

    if (1) then
        puts('true$N');
        if (2) then
            puts('truetrue$N');
        else
            puts('truefalse$N');
        end_if;
        puts('true$N');
    else
        puts('false$N');
    end_if;
END_FUNCTION
