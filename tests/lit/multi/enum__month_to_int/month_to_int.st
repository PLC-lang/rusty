FUNCTION main : DINT
    printf('%d', month_to_int(MONTH#June));
END_FUNCTION
 

FUNCTION month_to_int : INT
    VAR_INPUT
        month_var : MONTH;
    END_VAR

    CASE month_var OF
        MONTH.January:      month_to_int := 1;
        MONTH.February:     month_to_int := 2;
        MONTH.March:        month_to_int := 3;
        MONTH.April:        month_to_int := 4;
        MONTH.May:          month_to_int := 5;
        MONTH.June:         month_to_int := 6;
        MONTH.July:         month_to_int := 7;
        MONTH.August:       month_to_int := 8;
        MONTH.September:    month_to_int := 9;
        MONTH.October:      month_to_int := 10;
        MONTH.November:     month_to_int := 11;
        MONTH.December:     month_to_int := 12;
        else                month_to_int := 0;
    END_CASE;
END_FUNCTION
