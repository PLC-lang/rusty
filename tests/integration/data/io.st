PROGRAM a
    VAR
        binvar      AT %IX7.8 : BOOL;   
        sinvar      AT %IB4.8 : BYTE;
        winvar      AT %IW3.1 : WORD;
        dinvar      AT %ID1.7 : DWORD;
        boutvar     AT %QX7.7 : BOOL;
        soutvar     AT %QB1.6 : BYTE;
        woutvar     AT %QW2.4 : WORD;
        doutvar     AT %QD1.4 : DWORD;
        bmemvar     AT %MX7.8 : BOOL;
        smemvar     AT %MB5.8 : BYTE;
        wmemvar     AT %MW4.2 : WORD;
        dmemvar     AT %MD8.3 : DWORD;
        linvar      AT %IL9.0 : LWORD;
        loutvar     AT %QL4.2 : LWORD;
        lmemvar     AT %ML9.6 : LWORD;
    END_VAR
END_PROGRAM
