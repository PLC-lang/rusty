{external} 
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        i : INT;
        j : INT;
    END_VAR
	puts('hello, world!$N');

    if (1) then
        puts('true$N');
        if (0) then
            puts('truetrue$N');
        elsif (0) then
            puts('truefalse$N');
        else
            puts('truefalse$N');
        end_if;
        puts('true$N');
    else
        puts('false$N');
    end_if;

    CASE 1 OF
    1:
        puts('Case 1$N');
    2:
        puts('Case 2$N');
    3:
        puts('Case 3$N');
    ELSE
        puts('Default case$N');
    END_CASE;

    CASE 1 OF
    2:
        puts('Case 2$N');
    ELSE
        puts('Default case$N');
    END_CASE;

    if (0) then
        puts('true$N');
    elsif (0) then
        puts('false$N');
    elsif (1) then
        if (1) then
            puts('true$N');
        elsif (0) then
            puts('false$N');
        else
            puts('false$N');
        end_if
        puts('true$N');
    else
        // puts('false$N');
    end_if;

    FOR i := 1 TO 10 DO
        FOR j := 2 TO 3 DO
            puts('This is an inner loop iteration');
        END_FOR;
        puts('This is a loop iteration $N');
    END_FOR;

    i := 0;
    j := 0;

    WHILE i < 10 DO
        puts('This is while loop iteration $N');
        i := i + 1;


        WHILE j < 1 DO
            puts('This is an inner while loop iteration $N');
            j := j + 1;
        END_WHILE;

    END_WHILE;

    i := 0;

    REPEAT
        puts('This is a repeat loop iteration $N');
        i := i + 1;
    UNTIL i >= 10 END_REPEAT;

END_FUNCTION
