// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    VAR_INPUT
        fb_input_sint   : SINT;
        fb_input_int    : INT;
        fb_input_dint   : DINT;
        fb_input_lint   : LINT := 4;
        fb_input_usint  : USINT;
        fb_input_uint   : UINT;
        fb_input_udint  : UDINT := 7;
        fb_input_ulint  : ULINT;
        fb_input_word   : WORD := 16#9;
        fb_input_dword  : DWORD;
        fb_input_lword  : LWORD;
        fb_input_bool   : BOOL := TRUE;
        fb_input_time   : TIME;
        fb_input_ltime  : LTIME := LT#13s;
    END_VAR
    VAR_OUTPUT
        fb_output_sint   : SINT := 1;
        fb_output_int    : INT;
        fb_output_dint   : DINT;
        fb_output_lint   : LINT;
        fb_output_usint  : USINT;
        fb_output_uint   : UINT := 6;
        fb_output_udint  : UDINT;
        fb_output_ulint  : ULINT;
        fb_output_word   : WORD;
        fb_output_dword  : DWORD;
        fb_output_lword  : LWORD := 16#B;
        fb_output_bool   : BOOL;
        fb_output_time   : TIME := T#12s;
        fb_output_ltime  : LTIME;
    END_VAR
    VAR PUBLIC
        fb_public_sint   : SINT;
        fb_public_int    : INT := 2;
        fb_public_dint   : DINT;
        fb_public_lint   : LINT;
        fb_public_usint  : USINT := 5;
        fb_public_uint   : UINT;
        fb_public_udint  : UDINT;
        fb_public_ulint  : ULINT;
        fb_public_word   : WORD;
        fb_public_dword  : DWORD := 16#A;
        fb_public_lword  : LWORD;
        fb_public_bool   : BOOL := TRUE;
        fb_public_time   : TIME;
        fb_public_ltime  : LTIME;
    END_VAR
    VAR PROTECTED
        fb_protected_sint   : SINT;
        fb_protected_int    : INT;
        fb_protected_dint   : DINT := 3;
        fb_protected_lint   : LINT;
        fb_protected_usint  : USINT;
        fb_protected_uint   : UINT;
        fb_protected_udint  : UDINT;
        fb_protected_ulint  : ULINT := 8;
        fb_protected_word   : WORD;
        fb_protected_dword  : DWORD;
        fb_protected_lword  : LWORD;
        fb_protected_bool   : BOOL;
        fb_protected_time   : TIME := T#42ms;
        fb_protected_ltime  : LTIME;
    END_VAR
    VAR INTERNAL
        fb_internal_sint   : SINT := 11;
        fb_internal_int    : INT;
        fb_internal_dint   : DINT;
        fb_internal_lint   : LINT;
        fb_internal_usint  : USINT := 12;
        fb_internal_uint   : UINT;
        fb_internal_udint  : UDINT;
        fb_internal_ulint  : ULINT;
        fb_internal_word   : WORD := 16#D;
        fb_internal_dword  : DWORD;
        fb_internal_lword  : LWORD;
        fb_internal_bool   : BOOL := TRUE;
        fb_internal_time   : TIME;
        fb_internal_ltime  : LTIME := LT#1s;
    END_VAR
    VAR PRIVATE
        fb_private_sint   : SINT;
        fb_private_int    : INT := 21;
        fb_private_dint   : DINT;
        fb_private_lint   : LINT;
        fb_private_usint  : USINT;
        fb_private_uint   : UINT := 22;
        fb_private_udint  : UDINT;
        fb_private_ulint  : ULINT;
        fb_private_word   : WORD;
        fb_private_dword  : DWORD := 16#17;
        fb_private_lword  : LWORD;
        fb_private_bool   : BOOL;
        fb_private_time   : TIME;
        fb_private_ltime  : LTIME;
    END_VAR
    VAR CONSTANT
        fb_constant_sint   : SINT := 1;
        fb_constant_int    : INT := 2;
        fb_constant_dint   : DINT := 3;
        fb_constant_lint   : LINT := 4;
        fb_constant_usint  : USINT := 5;
        fb_constant_uint   : UINT := 6;
        fb_constant_udint  : UDINT := 7;
        fb_constant_ulint  : ULINT := 8;
        fb_constant_word   : WORD := 16#9;
        fb_constant_dword  : DWORD := 16#A;
        fb_constant_lword  : LWORD := 16#B;
        fb_constant_bool   : BOOL := TRUE;
        fb_constant_time   : TIME := T#13ms;
        fb_constant_ltime  : LTIME := LT#14us;
    END_VAR
    METHOD myMethod
        VAR_INPUT
            m_input_sint   : SINT;
            m_input_int    : INT := 2;
            m_input_dint   : DINT;
            m_input_lint   : LINT;
            m_input_usint  : USINT;
            m_input_uint   : UINT := 6;
            m_input_udint  : UDINT;
            m_input_ulint  : ULINT;
            m_input_word   : WORD;
            m_input_dword  : DWORD := 16#A;
            m_input_lword  : LWORD;
            m_input_bool   : BOOL;
            m_input_time   : TIME := T#12s;
            m_input_ltime  : LTIME;
        END_VAR
        VAR_OUTPUT
            m_output_sint   : SINT;
            m_output_int    : INT;
            m_output_dint   : DINT := 3;
            m_output_lint   : LINT;
            m_output_usint  : USINT;
            m_output_uint   : UINT;
            m_output_udint  : UDINT;
            m_output_ulint  : ULINT := 8;
            m_output_word   : WORD;
            m_output_dword  : DWORD;
            m_output_lword  : LWORD := 16#B;
            m_output_bool   : BOOL := TRUE;
            m_output_time   : TIME;
            m_output_ltime  : LTIME := LT#13us;
        END_VAR
        VAR
            m_var_sint   : SINT := 1;
            m_var_int    : INT;
            m_var_dint   : DINT;
            m_var_lint   : LINT;
            m_var_usint  : USINT := 5;
            m_var_uint   : UINT;
            m_var_udint  : UDINT;
            m_var_ulint  : ULINT;
            m_var_word   : WORD := 16#9;
            m_var_dword  : DWORD;
            m_var_lword  : LWORD;
            m_var_bool   : BOOL := TRUE;
            m_var_time   : TIME := T#12ms;
            m_var_ltime  : LTIME := LT#13ns;
        END_VAR

        PRINTF('Checking fb variable after call of method:$N');
        PRINTF('- fb_input_sint: %hhd$N', fb_input_sint);
        PRINTF('- fb_input_int: %hd$N', fb_input_int);
        PRINTF('- fb_input_dint: %d$N', fb_input_dint);
        PRINTF('- fb_input_lint: %lld$N', fb_input_lint);
        PRINTF('- fb_input_usint: %hhu$N', fb_input_usint);
        PRINTF('- fb_input_uint: %hu$N', fb_input_uint);
        PRINTF('- fb_input_udint: %u$N', fb_input_udint);
        PRINTF('- fb_input_ulint: %llu$N', fb_input_ulint);
        PRINTF('- fb_input_word: %hu$N', fb_input_word);
        PRINTF('- fb_input_dword: %u$N', fb_input_dword);
        PRINTF('- fb_input_lword: %llu$N', fb_input_lword);
        PRINTF('- fb_input_bool: %hhu$N', fb_input_bool);
        PRINTF('- fb_input_time: %llu$N', fb_input_time);
        PRINTF('- fb_input_ltime: %llu$N', fb_input_ltime);

        PRINTF('- fb_output_sint: %hhd$N', fb_output_sint);
        PRINTF('- fb_output_int: %hd$N', fb_output_int);
        PRINTF('- fb_output_dint: %d$N', fb_output_dint);
        PRINTF('- fb_output_lint: %lld$N', fb_output_lint);
        PRINTF('- fb_output_usint: %hhu$N', fb_output_usint);
        PRINTF('- fb_output_uint: %hu$N', fb_output_uint);
        PRINTF('- fb_output_udint: %u$N', fb_output_udint);
        PRINTF('- fb_output_ulint: %llu$N', fb_output_ulint);
        PRINTF('- fb_output_word: %hu$N', fb_output_word);
        PRINTF('- fb_output_dword: %u$N', fb_output_dword);
        PRINTF('- fb_output_lword: %llu$N', fb_output_lword);
        PRINTF('- fb_output_bool: %hhu$N', fb_output_bool);
        PRINTF('- fb_output_time: %llu$N', fb_output_time);
        PRINTF('- fb_output_ltime: %llu$N', fb_output_ltime);

        PRINTF('- fb_public_sint: %hhd$N', fb_public_sint);
        PRINTF('- fb_public_int: %hd$N', fb_public_int);
        PRINTF('- fb_public_dint: %d$N', fb_public_dint);
        PRINTF('- fb_public_lint: %lld$N', fb_public_lint);
        PRINTF('- fb_public_usint: %hhu$N', fb_public_usint);
        PRINTF('- fb_public_uint: %hu$N', fb_public_uint);
        PRINTF('- fb_public_udint: %u$N', fb_public_udint);
        PRINTF('- fb_public_ulint: %llu$N', fb_public_ulint);
        PRINTF('- fb_public_word: %hu$N', fb_public_word);
        PRINTF('- fb_public_dword: %u$N', fb_public_dword);
        PRINTF('- fb_public_lword: %llu$N', fb_public_lword);
        PRINTF('- fb_public_bool: %hhu$N', fb_public_bool);
        PRINTF('- fb_public_time: %llu$N', fb_public_time);
        PRINTF('- fb_public_ltime: %llu$N', fb_public_ltime);

        PRINTF('- fb_protected_sint: %hhd$N', fb_protected_sint);
        PRINTF('- fb_protected_int: %hd$N', fb_protected_int);
        PRINTF('- fb_protected_dint: %d$N', fb_protected_dint);
        PRINTF('- fb_protected_lint: %lld$N', fb_protected_lint);
        PRINTF('- fb_protected_usint: %hhu$N', fb_protected_usint);
        PRINTF('- fb_protected_uint: %hu$N', fb_protected_uint);
        PRINTF('- fb_protected_udint: %u$N', fb_protected_udint);
        PRINTF('- fb_protected_ulint: %llu$N', fb_protected_ulint);
        PRINTF('- fb_protected_word: %hu$N', fb_protected_word);
        PRINTF('- fb_protected_dword: %u$N', fb_protected_dword);
        PRINTF('- fb_protected_lword: %llu$N', fb_protected_lword);
        PRINTF('- fb_protected_bool: %hhu$N', fb_protected_bool);
        PRINTF('- fb_protected_time: %llu$N', fb_protected_time);
        PRINTF('- fb_protected_ltime: %llu$N', fb_protected_ltime);

        PRINTF('- fb_internal_sint: %hhd$N', fb_internal_sint);
        PRINTF('- fb_internal_int: %hd$N', fb_internal_int);
        PRINTF('- fb_internal_dint: %d$N', fb_internal_dint);
        PRINTF('- fb_internal_lint: %lld$N', fb_internal_lint);
        PRINTF('- fb_internal_usint: %hhu$N', fb_internal_usint);
        PRINTF('- fb_internal_uint: %hu$N', fb_internal_uint);
        PRINTF('- fb_internal_udint: %u$N', fb_internal_udint);
        PRINTF('- fb_internal_ulint: %llu$N', fb_internal_ulint);
        PRINTF('- fb_internal_word: %hu$N', fb_internal_word);
        PRINTF('- fb_internal_dword: %u$N', fb_internal_dword);
        PRINTF('- fb_internal_lword: %llu$N', fb_internal_lword);
        PRINTF('- fb_internal_bool: %hhu$N', fb_internal_bool);
        PRINTF('- fb_internal_time: %llu$N', fb_internal_time);
        PRINTF('- fb_internal_ltime: %llu$N', fb_internal_ltime);

        PRINTF('- fb_private_sint: %hhd$N', fb_private_sint);
        PRINTF('- fb_private_int: %hd$N', fb_private_int);
        PRINTF('- fb_private_dint: %d$N', fb_private_dint);
        PRINTF('- fb_private_lint: %lld$N', fb_private_lint);
        PRINTF('- fb_private_usint: %hhu$N', fb_private_usint);
        PRINTF('- fb_private_uint: %hu$N', fb_private_uint);
        PRINTF('- fb_private_udint: %u$N', fb_private_udint);
        PRINTF('- fb_private_ulint: %llu$N', fb_private_ulint);
        PRINTF('- fb_private_word: %hu$N', fb_private_word);
        PRINTF('- fb_private_dword: %u$N', fb_private_dword);
        PRINTF('- fb_private_lword: %llu$N', fb_private_lword);
        PRINTF('- fb_private_bool: %hhu$N', fb_private_bool);
        PRINTF('- fb_private_time: %llu$N', fb_private_time);
        PRINTF('- fb_private_ltime: %llu$N', fb_private_ltime);

        PRINTF('- fb_constant_sint: %hhd$N', fb_constant_sint);
        PRINTF('- fb_constant_int: %hd$N', fb_constant_int);
        PRINTF('- fb_constant_dint: %d$N', fb_constant_dint);
        PRINTF('- fb_constant_lint: %lld$N', fb_constant_lint);
        PRINTF('- fb_constant_usint: %hhu$N', fb_constant_usint);
        PRINTF('- fb_constant_uint: %hu$N', fb_constant_uint);
        PRINTF('- fb_constant_udint: %u$N', fb_constant_udint);
        PRINTF('- fb_constant_ulint: %llu$N', fb_constant_ulint);
        PRINTF('- fb_constant_word: %hu$N', fb_constant_word);
        PRINTF('- fb_constant_dword: %u$N', fb_constant_dword);
        PRINTF('- fb_constant_lword: %llu$N', fb_constant_lword);
        PRINTF('- fb_constant_bool: %hhu$N', fb_constant_bool);
        PRINTF('- fb_constant_time: %llu$N', fb_constant_time);
        PRINTF('- fb_constant_ltime: %llu$N', fb_constant_ltime);

        PRINTF('Checking method variable after call:$N');
        PRINTF('- m_input_sint: %hhd$N', m_input_sint);
        PRINTF('- m_input_int: %hd$N', m_input_int);
        PRINTF('- m_input_dint: %d$N', m_input_dint);
        PRINTF('- m_input_lint: %lld$N', m_input_lint);
        PRINTF('- m_input_usint: %hhu$N', m_input_usint);
        PRINTF('- m_input_uint: %hu$N', m_input_uint);
        PRINTF('- m_input_udint: %u$N', m_input_udint);
        PRINTF('- m_input_ulint: %llu$N', m_input_ulint);
        PRINTF('- m_input_word: %hu$N', m_input_word);
        PRINTF('- m_input_dword: %u$N', m_input_dword);
        PRINTF('- m_input_lword: %llu$N', m_input_lword);
        PRINTF('- m_input_bool: %hhu$N', m_input_bool);
        PRINTF('- m_input_time: %llu$N', m_input_time);
        PRINTF('- m_input_ltime: %llu$N', m_input_ltime);

        PRINTF('- m_output_sint: %hhd$N', m_output_sint);
        PRINTF('- m_output_int: %hd$N', m_output_int);
        PRINTF('- m_output_dint: %d$N', m_output_dint);
        PRINTF('- m_output_lint: %lld$N', m_output_lint);
        PRINTF('- m_output_usint: %hhu$N', m_output_usint);
        PRINTF('- m_output_uint: %hu$N', m_output_uint);
        PRINTF('- m_output_udint: %u$N', m_output_udint);
        PRINTF('- m_output_ulint: %llu$N', m_output_ulint);
        PRINTF('- m_output_word: %hu$N', m_output_word);
        PRINTF('- m_output_dword: %u$N', m_output_dword);
        PRINTF('- m_output_lword: %llu$N', m_output_lword);
        PRINTF('- m_output_bool: %hhu$N', m_output_bool);
        PRINTF('- m_output_time: %llu$N', m_output_time);
        PRINTF('- m_output_ltime: %llu$N', m_output_ltime);

        PRINTF('- m_var_sint: %hhd$N', m_var_sint);
        PRINTF('- m_var_int: %hd$N', m_var_int);
        PRINTF('- m_var_dint: %d$N', m_var_dint);
        PRINTF('- m_var_lint: %lld$N', m_var_lint);
        PRINTF('- m_var_usint: %hhu$N', m_var_usint);
        PRINTF('- m_var_uint: %hu$N', m_var_uint);
        PRINTF('- m_var_udint: %u$N', m_var_udint);
        PRINTF('- m_var_ulint: %llu$N', m_var_ulint);
        PRINTF('- m_var_word: %hu$N', m_var_word);
        PRINTF('- m_var_dword: %u$N', m_var_dword);
        PRINTF('- m_var_lword: %llu$N', m_var_lword);
        PRINTF('- m_var_bool: %hhu$N', m_var_bool);
        PRINTF('- m_var_time: %llu$N', m_var_time);
        PRINTF('- m_var_ltime: %llu$N', m_var_ltime);

        fb_output_sint := -1;
        fb_output_int := -2;
        fb_output_dint := -3;
        fb_output_lint := -4;
        fb_output_usint := 1;
        fb_output_uint := 2;
        fb_output_udint := 3;
        fb_output_ulint := 4;
        fb_output_word := 16#1;
        fb_output_dword := 16#2;
        fb_output_lword := 16#3;
        fb_output_bool := TRUE;
        fb_output_time := t#1ms;
        fb_output_ltime := lt#1ns;

        fb_public_sint := -1;
        fb_public_int := -2;
        fb_public_dint := -3;
        fb_public_lint := -4;
        fb_public_usint := 1;
        fb_public_uint := 2;
        fb_public_udint := 3;
        fb_public_ulint := 4;
        fb_public_word := 16#1;
        fb_public_dword := 16#2;
        fb_public_lword := 16#3;
        fb_public_bool := TRUE;
        fb_public_time := t#1ms;
        fb_public_ltime := lt#1ns;

        fb_protected_sint := -1;
        fb_protected_int := -2;
        fb_protected_dint := -3;
        fb_protected_lint := -4;
        fb_protected_usint := 1;
        fb_protected_uint := 2;
        fb_protected_udint := 3;
        fb_protected_ulint := 4;
        fb_protected_word := 16#1;
        fb_protected_dword := 16#2;
        fb_protected_lword := 16#3;
        fb_protected_bool := TRUE;
        fb_protected_time := t#1ms;
        fb_protected_ltime := lt#1ns;

        fb_internal_sint := -1;
        fb_internal_int := -2;
        fb_internal_dint := -3;
        fb_internal_lint := -4;
        fb_internal_usint := 1;
        fb_internal_uint := 2;
        fb_internal_udint := 3;
        fb_internal_ulint := 4;
        fb_internal_word := 16#1;
        fb_internal_dword := 16#2;
        fb_internal_lword := 16#3;
        fb_internal_bool := TRUE;
        fb_internal_time := t#1ms;
        fb_internal_ltime := lt#1ns;

        fb_private_sint := -1;
        fb_private_int := -2;
        fb_private_dint := -3;
        fb_private_lint := -4;
        fb_private_usint := 1;
        fb_private_uint := 2;
        fb_private_udint := 3;
        fb_private_ulint := 4;
        fb_private_word := 16#1;
        fb_private_dword := 16#2;
        fb_private_lword := 16#3;
        fb_private_bool := TRUE;
        fb_private_time := t#1ms;
        fb_private_ltime := lt#1ns;

        m_var_sint := m_input_sint;
        m_var_int := m_input_int;
        m_var_dint := m_input_dint;
        m_var_lint := m_input_lint;
        m_var_usint := m_input_usint;
        m_var_uint := m_input_uint;
        m_var_udint := m_input_udint;
        m_var_ulint := m_input_ulint;
        m_var_word := m_input_word;
        m_var_dword := m_input_dword;
        m_var_lword := m_input_lword;
        m_var_bool := m_input_bool;
        m_var_time := m_input_time;
        m_var_ltime := m_input_ltime;

        m_output_sint := m_var_sint;
        m_output_int := m_var_int;
        m_output_dint := m_var_dint;
        m_output_lint := m_var_lint;
        m_output_usint := m_var_usint;
        m_output_uint := m_var_uint;
        m_output_udint := m_var_udint;
        m_output_ulint := m_var_ulint;
        m_output_word := m_var_word;
        m_output_dword := m_var_dword;
        m_output_lword := m_var_lword;
        m_output_bool := m_var_bool;
        m_output_time := m_var_time;
        m_output_ltime := m_var_ltime;
    END_METHOD
    METHOD myMethod_sint : SINT
        VAR_INPUT
            m_input_sint   : SINT;
        END_VAR
        myMethod_sint := m_input_sint;
    END_METHOD
    METHOD myMethod_int : INT
        VAR_INPUT
            m_input_int   : INT;
        END_VAR
        myMethod_int := m_input_int;
    END_METHOD
    METHOD myMethod_dint : DINT
        VAR_INPUT
            m_input_dint   : DINT;
        END_VAR
        myMethod_dint := m_input_dint;
    END_METHOD
    METHOD myMethod_lint : LINT
        VAR_INPUT
            m_input_lint   : LINT;
        END_VAR
        myMethod_lint := m_input_lint;
    END_METHOD
    METHOD myMethod_usint : USINT
        VAR_INPUT
            m_input_usint   : USINT;
        END_VAR
        myMethod_usint := m_input_usint;
    END_METHOD
    METHOD myMethod_uint : UINT
        VAR_INPUT
            m_input_uint   : UINT;
        END_VAR
        myMethod_uint := m_input_uint;
    END_METHOD
    METHOD myMethod_udint : UDINT
        VAR_INPUT
            m_input_udint   : UDINT;
        END_VAR
        myMethod_udint := m_input_udint;
    END_METHOD
    METHOD myMethod_ulint : ULINT
        VAR_INPUT
            m_input_ulint   : ULINT;
        END_VAR
        myMethod_ulint := m_input_ulint;
    END_METHOD
    METHOD myMethod_word : WORD
        VAR_INPUT
            m_input_word   : WORD;
        END_VAR
        myMethod_word := m_input_word;
    END_METHOD
    METHOD myMethod_dword : DWORD
        VAR_INPUT
            m_input_dword   : DWORD;
        END_VAR
        myMethod_dword := m_input_dword;
    END_METHOD
    METHOD myMethod_lword : LWORD
        VAR_INPUT
            m_input_lword   : LWORD;
        END_VAR
        myMethod_lword := m_input_lword;
    END_METHOD
    METHOD myMethod_bool : BOOL
        VAR_INPUT
            m_input_bool   : BOOL;
        END_VAR
        myMethod_bool := m_input_bool;
    END_METHOD
    METHOD myMethod_time : TIME
        VAR_INPUT
            m_input_time   : TIME;
        END_VAR
        myMethod_time := m_input_time;
    END_METHOD
    METHOD myMethod_ltime : LTIME
        VAR_INPUT
            m_input_ltime   : LTIME;
        END_VAR
        myMethod_ltime := m_input_ltime;
    END_METHOD
    METHOD myMethod_arr_int : ARRAY[1..5] OF INT
        VAR_INPUT
            m_input_arr_idx1   : INT;
            m_input_arr_idx3   : INT;
        END_VAR
        myMethod_arr_int[1] := m_input_arr_idx1;
        myMethod_arr_int[3] := m_input_arr_idx3;
    END_METHOD
    METHOD myMethod_bits
        VAR_INPUT
            m_input_byte   : BYTE;
        END_VAR
        VAR_OUTPUT
            m_output_bits  : ARRAY[1..8] OF BOOL;
        END_VAR
        m_output_bits[1] := m_input_byte.0;
        m_output_bits[2] := m_input_byte.1;
        m_output_bits[3] := m_input_byte.2;
        m_output_bits[4] := m_input_byte.3;
        m_output_bits[5] := m_input_byte.4;
        m_output_bits[6] := m_input_byte.5;
        m_output_bits[7] := m_input_byte.6;
        m_output_bits[8] := m_input_byte.7;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 extends fb
    METHOD myMethod2
        VAR_INPUT
            m_input_sint   : SINT;
            m_input_int    : INT := 2;
            m_input_dint   : DINT;
            m_input_lint   : LINT;
            m_input_usint  : USINT;
            m_input_uint   : UINT := 6;
            m_input_udint  : UDINT;
            m_input_ulint  : ULINT;
            m_input_word   : WORD;
            m_input_dword  : DWORD := 16#A;
            m_input_lword  : LWORD;
            m_input_bool   : BOOL;
            m_input_time   : TIME := T#12s;
            m_input_ltime  : LTIME;
        END_VAR
        VAR_OUTPUT
            m_output_sint   : SINT;
            m_output_int    : INT;
            m_output_dint   : DINT := 3;
            m_output_lint   : LINT;
            m_output_usint  : USINT;
            m_output_uint   : UINT;
            m_output_udint  : UDINT;
            m_output_ulint  : ULINT := 8;
            m_output_word   : WORD;
            m_output_dword  : DWORD;
            m_output_lword  : LWORD := 16#B;
            m_output_bool   : BOOL := TRUE;
            m_output_time   : TIME;
            m_output_ltime  : LTIME := LT#13us;
        END_VAR
        VAR
            m_var_sint   : SINT := 1;
            m_var_int    : INT;
            m_var_dint   : DINT;
            m_var_lint   : LINT;
            m_var_usint  : USINT := 5;
            m_var_uint   : UINT;
            m_var_udint  : UDINT;
            m_var_ulint  : ULINT;
            m_var_word   : WORD := 16#9;
            m_var_dword  : DWORD;
            m_var_lword  : LWORD;
            m_var_bool   : BOOL := TRUE;
            m_var_time   : TIME := T#12ms;
            m_var_ltime  : LTIME := LT#13ns;
        END_VAR

        PRINTF('Checking fb variable after call of method2:$N');
        PRINTF('- fb_input_sint: %hhd$N', fb_input_sint);
        PRINTF('- fb_input_int: %hd$N', fb_input_int);
        PRINTF('- fb_input_dint: %d$N', fb_input_dint);
        PRINTF('- fb_input_lint: %lld$N', fb_input_lint);
        PRINTF('- fb_input_usint: %hhu$N', fb_input_usint);
        PRINTF('- fb_input_uint: %hu$N', fb_input_uint);
        PRINTF('- fb_input_udint: %u$N', fb_input_udint);
        PRINTF('- fb_input_ulint: %llu$N', fb_input_ulint);
        PRINTF('- fb_input_word: %hu$N', fb_input_word);
        PRINTF('- fb_input_dword: %u$N', fb_input_dword);
        PRINTF('- fb_input_lword: %llu$N', fb_input_lword);
        PRINTF('- fb_input_bool: %hhu$N', fb_input_bool);
        PRINTF('- fb_input_time: %llu$N', fb_input_time);
        PRINTF('- fb_input_ltime: %llu$N', fb_input_ltime);

        PRINTF('- fb_output_sint: %hhd$N', fb_output_sint);
        PRINTF('- fb_output_int: %hd$N', fb_output_int);
        PRINTF('- fb_output_dint: %d$N', fb_output_dint);
        PRINTF('- fb_output_lint: %lld$N', fb_output_lint);
        PRINTF('- fb_output_usint: %hhu$N', fb_output_usint);
        PRINTF('- fb_output_uint: %hu$N', fb_output_uint);
        PRINTF('- fb_output_udint: %u$N', fb_output_udint);
        PRINTF('- fb_output_ulint: %llu$N', fb_output_ulint);
        PRINTF('- fb_output_word: %hu$N', fb_output_word);
        PRINTF('- fb_output_dword: %u$N', fb_output_dword);
        PRINTF('- fb_output_lword: %llu$N', fb_output_lword);
        PRINTF('- fb_output_bool: %hhu$N', fb_output_bool);
        PRINTF('- fb_output_time: %llu$N', fb_output_time);
        PRINTF('- fb_output_ltime: %llu$N', fb_output_ltime);

        PRINTF('- fb_public_sint: %hhd$N', fb_public_sint);
        PRINTF('- fb_public_int: %hd$N', fb_public_int);
        PRINTF('- fb_public_dint: %d$N', fb_public_dint);
        PRINTF('- fb_public_lint: %lld$N', fb_public_lint);
        PRINTF('- fb_public_usint: %hhu$N', fb_public_usint);
        PRINTF('- fb_public_uint: %hu$N', fb_public_uint);
        PRINTF('- fb_public_udint: %u$N', fb_public_udint);
        PRINTF('- fb_public_ulint: %llu$N', fb_public_ulint);
        PRINTF('- fb_public_word: %hu$N', fb_public_word);
        PRINTF('- fb_public_dword: %u$N', fb_public_dword);
        PRINTF('- fb_public_lword: %llu$N', fb_public_lword);
        PRINTF('- fb_public_bool: %hhu$N', fb_public_bool);
        PRINTF('- fb_public_time: %llu$N', fb_public_time);
        PRINTF('- fb_public_ltime: %llu$N', fb_public_ltime);

        PRINTF('- fb_protected_sint: %hhd$N', fb_protected_sint);
        PRINTF('- fb_protected_int: %hd$N', fb_protected_int);
        PRINTF('- fb_protected_dint: %d$N', fb_protected_dint);
        PRINTF('- fb_protected_lint: %lld$N', fb_protected_lint);
        PRINTF('- fb_protected_usint: %hhu$N', fb_protected_usint);
        PRINTF('- fb_protected_uint: %hu$N', fb_protected_uint);
        PRINTF('- fb_protected_udint: %u$N', fb_protected_udint);
        PRINTF('- fb_protected_ulint: %llu$N', fb_protected_ulint);
        PRINTF('- fb_protected_word: %hu$N', fb_protected_word);
        PRINTF('- fb_protected_dword: %u$N', fb_protected_dword);
        PRINTF('- fb_protected_lword: %llu$N', fb_protected_lword);
        PRINTF('- fb_protected_bool: %hhu$N', fb_protected_bool);
        PRINTF('- fb_protected_time: %llu$N', fb_protected_time);
        PRINTF('- fb_protected_ltime: %llu$N', fb_protected_ltime);

        PRINTF('- fb_internal_sint: %hhd$N', fb_internal_sint);
        PRINTF('- fb_internal_int: %hd$N', fb_internal_int);
        PRINTF('- fb_internal_dint: %d$N', fb_internal_dint);
        PRINTF('- fb_internal_lint: %lld$N', fb_internal_lint);
        PRINTF('- fb_internal_usint: %hhu$N', fb_internal_usint);
        PRINTF('- fb_internal_uint: %hu$N', fb_internal_uint);
        PRINTF('- fb_internal_udint: %u$N', fb_internal_udint);
        PRINTF('- fb_internal_ulint: %llu$N', fb_internal_ulint);
        PRINTF('- fb_internal_word: %hu$N', fb_internal_word);
        PRINTF('- fb_internal_dword: %u$N', fb_internal_dword);
        PRINTF('- fb_internal_lword: %llu$N', fb_internal_lword);
        PRINTF('- fb_internal_bool: %hhu$N', fb_internal_bool);
        PRINTF('- fb_internal_time: %llu$N', fb_internal_time);
        PRINTF('- fb_internal_ltime: %llu$N', fb_internal_ltime);

        PRINTF('- fb_constant_sint: %hhd$N', fb_constant_sint);
        PRINTF('- fb_constant_int: %hd$N', fb_constant_int);
        PRINTF('- fb_constant_dint: %d$N', fb_constant_dint);
        PRINTF('- fb_constant_lint: %lld$N', fb_constant_lint);
        PRINTF('- fb_constant_usint: %hhu$N', fb_constant_usint);
        PRINTF('- fb_constant_uint: %hu$N', fb_constant_uint);
        PRINTF('- fb_constant_udint: %u$N', fb_constant_udint);
        PRINTF('- fb_constant_ulint: %llu$N', fb_constant_ulint);
        PRINTF('- fb_constant_word: %hu$N', fb_constant_word);
        PRINTF('- fb_constant_dword: %u$N', fb_constant_dword);
        PRINTF('- fb_constant_lword: %llu$N', fb_constant_lword);
        PRINTF('- fb_constant_bool: %hhu$N', fb_constant_bool);
        PRINTF('- fb_constant_time: %llu$N', fb_constant_time);
        PRINTF('- fb_constant_ltime: %llu$N', fb_constant_ltime);

        PRINTF('Checking method2 variable after call:$N');
        PRINTF('- m_input_sint: %hhd$N', m_input_sint);
        PRINTF('- m_input_int: %hd$N', m_input_int);
        PRINTF('- m_input_dint: %d$N', m_input_dint);
        PRINTF('- m_input_lint: %lld$N', m_input_lint);
        PRINTF('- m_input_usint: %hhu$N', m_input_usint);
        PRINTF('- m_input_uint: %hu$N', m_input_uint);
        PRINTF('- m_input_udint: %u$N', m_input_udint);
        PRINTF('- m_input_ulint: %llu$N', m_input_ulint);
        PRINTF('- m_input_word: %hu$N', m_input_word);
        PRINTF('- m_input_dword: %u$N', m_input_dword);
        PRINTF('- m_input_lword: %llu$N', m_input_lword);
        PRINTF('- m_input_bool: %hhu$N', m_input_bool);
        PRINTF('- m_input_time: %llu$N', m_input_time);
        PRINTF('- m_input_ltime: %llu$N', m_input_ltime);

        PRINTF('- m_output_sint: %hhd$N', m_output_sint);
        PRINTF('- m_output_int: %hd$N', m_output_int);
        PRINTF('- m_output_dint: %d$N', m_output_dint);
        PRINTF('- m_output_lint: %lld$N', m_output_lint);
        PRINTF('- m_output_usint: %hhu$N', m_output_usint);
        PRINTF('- m_output_uint: %hu$N', m_output_uint);
        PRINTF('- m_output_udint: %u$N', m_output_udint);
        PRINTF('- m_output_ulint: %llu$N', m_output_ulint);
        PRINTF('- m_output_word: %hu$N', m_output_word);
        PRINTF('- m_output_dword: %u$N', m_output_dword);
        PRINTF('- m_output_lword: %llu$N', m_output_lword);
        PRINTF('- m_output_bool: %hhu$N', m_output_bool);
        PRINTF('- m_output_time: %llu$N', m_output_time);
        PRINTF('- m_output_ltime: %llu$N', m_output_ltime);

        PRINTF('- m_var_sint: %hhd$N', m_var_sint);
        PRINTF('- m_var_int: %hd$N', m_var_int);
        PRINTF('- m_var_dint: %d$N', m_var_dint);
        PRINTF('- m_var_lint: %lld$N', m_var_lint);
        PRINTF('- m_var_usint: %hhu$N', m_var_usint);
        PRINTF('- m_var_uint: %hu$N', m_var_uint);
        PRINTF('- m_var_udint: %u$N', m_var_udint);
        PRINTF('- m_var_ulint: %llu$N', m_var_ulint);
        PRINTF('- m_var_word: %hu$N', m_var_word);
        PRINTF('- m_var_dword: %u$N', m_var_dword);
        PRINTF('- m_var_lword: %llu$N', m_var_lword);
        PRINTF('- m_var_bool: %hhu$N', m_var_bool);
        PRINTF('- m_var_time: %llu$N', m_var_time);
        PRINTF('- m_var_ltime: %llu$N', m_var_ltime);

        fb_output_sint := -1;
        fb_output_int := -2;
        fb_output_dint := -3;
        fb_output_lint := -4;
        fb_output_usint := 1;
        fb_output_uint := 2;
        fb_output_udint := 3;
        fb_output_ulint := 4;
        fb_output_word := 16#1;
        fb_output_dword := 16#2;
        fb_output_lword := 16#3;
        fb_output_bool := TRUE;
        fb_output_time := t#1ms;
        fb_output_ltime := lt#1ns;

        fb_public_sint := -1;
        fb_public_int := -2;
        fb_public_dint := -3;
        fb_public_lint := -4;
        fb_public_usint := 1;
        fb_public_uint := 2;
        fb_public_udint := 3;
        fb_public_ulint := 4;
        fb_public_word := 16#1;
        fb_public_dword := 16#2;
        fb_public_lword := 16#3;
        fb_public_bool := TRUE;
        fb_public_time := t#1ms;
        fb_public_ltime := lt#1ns;

        fb_protected_sint := -1;
        fb_protected_int := -2;
        fb_protected_dint := -3;
        fb_protected_lint := -4;
        fb_protected_usint := 1;
        fb_protected_uint := 2;
        fb_protected_udint := 3;
        fb_protected_ulint := 4;
        fb_protected_word := 16#1;
        fb_protected_dword := 16#2;
        fb_protected_lword := 16#3;
        fb_protected_bool := TRUE;
        fb_protected_time := t#1ms;
        fb_protected_ltime := lt#1ns;

        fb_internal_sint := -1;
        fb_internal_int := -2;
        fb_internal_dint := -3;
        fb_internal_lint := -4;
        fb_internal_usint := 1;
        fb_internal_uint := 2;
        fb_internal_udint := 3;
        fb_internal_ulint := 4;
        fb_internal_word := 16#1;
        fb_internal_dword := 16#2;
        fb_internal_lword := 16#3;
        fb_internal_bool := TRUE;
        fb_internal_time := t#1ms;
        fb_internal_ltime := lt#1ns;

        m_var_sint := m_input_sint;
        m_var_int := m_input_int;
        m_var_dint := m_input_dint;
        m_var_lint := m_input_lint;
        m_var_usint := m_input_usint;
        m_var_uint := m_input_uint;
        m_var_udint := m_input_udint;
        m_var_ulint := m_input_ulint;
        m_var_word := m_input_word;
        m_var_dword := m_input_dword;
        m_var_lword := m_input_lword;
        m_var_bool := m_input_bool;
        m_var_time := m_input_time;
        m_var_ltime := m_input_ltime;

        m_output_sint := m_var_sint;
        m_output_int := m_var_int;
        m_output_dint := m_var_dint;
        m_output_lint := m_var_lint;
        m_output_usint := m_var_usint;
        m_output_uint := m_var_uint;
        m_output_udint := m_var_udint;
        m_output_ulint := m_var_ulint;
        m_output_word := m_var_word;
        m_output_dword := m_var_dword;
        m_output_lword := m_var_lword;
        m_output_bool := m_var_bool;
        m_output_time := m_var_time;
        m_output_ltime := m_var_ltime;
    END_METHOD
END_FUNCTION_BLOCK
FUNCTION main
    VAR
        fb_instance : fb;
        fb2_instance : fb2;
        var_sint        : SINT;
        var_int         : INT;
        var_dint        : DINT;
        var_lint        : LINT;
        var_usint       : USINT;
        var_uint        : UINT;
        var_udint       : UDINT;
        var_ulint       : ULINT;
        var_word        : WORD;
        var_dword       : DWORD;
        var_lword       : LWORD;
        var_bool        : BOOL;
        var_time        : TIME;
        var_ltime       : LTIME;
        var_arr_int     : ARRAY[1..5] OF INT;
        var_bits        : ARRAY[1..8] OF BOOL;
    END_VAR

    fb_instance.myMethod();

    //CHECK:Checking fb variable after call of method:
    //CHECK-NEXT:- fb_input_sint: 0
    //CHECK-NEXT:- fb_input_int: 0
    //CHECK-NEXT:- fb_input_dint: 0
    //CHECK-NEXT:- fb_input_lint: 4
    //CHECK-NEXT:- fb_input_usint: 0
    //CHECK-NEXT:- fb_input_uint: 0
    //CHECK-NEXT:- fb_input_udint: 7
    //CHECK-NEXT:- fb_input_ulint: 0
    //CHECK-NEXT:- fb_input_word: 9
    //CHECK-NEXT:- fb_input_dword: 0
    //CHECK-NEXT:- fb_input_lword: 0
    //CHECK-NEXT:- fb_input_bool: 1
    //CHECK-NEXT:- fb_input_time: 0
    //CHECK-NEXT:- fb_input_ltime: 13000000000

    //CHECK-NEXT:- fb_output_sint: 1
    //CHECK-NEXT:- fb_output_int: 0
    //CHECK-NEXT:- fb_output_dint: 0
    //CHECK-NEXT:- fb_output_lint: 0
    //CHECK-NEXT:- fb_output_usint: 0
    //CHECK-NEXT:- fb_output_uint: 6
    //CHECK-NEXT:- fb_output_udint: 0
    //CHECK-NEXT:- fb_output_ulint: 0
    //CHECK-NEXT:- fb_output_word: 0
    //CHECK-NEXT:- fb_output_dword: 0
    //CHECK-NEXT:- fb_output_lword: 11
    //CHECK-NEXT:- fb_output_bool: 0
    //CHECK-NEXT:- fb_output_time: 12000000000
    //CHECK-NEXT:- fb_output_ltime: 0

    //CHECK-NEXT:- fb_public_sint: 0
    //CHECK-NEXT:- fb_public_int: 2
    //CHECK-NEXT:- fb_public_dint: 0
    //CHECK-NEXT:- fb_public_lint: 0
    //CHECK-NEXT:- fb_public_usint: 5
    //CHECK-NEXT:- fb_public_uint: 0
    //CHECK-NEXT:- fb_public_udint: 0
    //CHECK-NEXT:- fb_public_ulint: 0
    //CHECK-NEXT:- fb_public_word: 0
    //CHECK-NEXT:- fb_public_dword: 10
    //CHECK-NEXT:- fb_public_lword: 0
    //CHECK-NEXT:- fb_public_bool: 1
    //CHECK-NEXT:- fb_public_time: 0
    //CHECK-NEXT:- fb_public_ltime: 0

    //CHECK-NEXT:- fb_protected_sint: 0
    //CHECK-NEXT:- fb_protected_int: 0
    //CHECK-NEXT:- fb_protected_dint: 3
    //CHECK-NEXT:- fb_protected_lint: 0
    //CHECK-NEXT:- fb_protected_usint: 0
    //CHECK-NEXT:- fb_protected_uint: 0
    //CHECK-NEXT:- fb_protected_udint: 0
    //CHECK-NEXT:- fb_protected_ulint: 8
    //CHECK-NEXT:- fb_protected_word: 0
    //CHECK-NEXT:- fb_protected_dword: 0
    //CHECK-NEXT:- fb_protected_lword: 0
    //CHECK-NEXT:- fb_protected_bool: 0
    //CHECK-NEXT:- fb_protected_time: 42000000
    //CHECK-NEXT:- fb_protected_ltime: 0

    //CHECK-NEXT:- fb_internal_sint: 11
    //CHECK-NEXT:- fb_internal_int: 0
    //CHECK-NEXT:- fb_internal_dint: 0
    //CHECK-NEXT:- fb_internal_lint: 0
    //CHECK-NEXT:- fb_internal_usint: 12
    //CHECK-NEXT:- fb_internal_uint: 0
    //CHECK-NEXT:- fb_internal_udint: 0
    //CHECK-NEXT:- fb_internal_ulint: 0
    //CHECK-NEXT:- fb_internal_word: 13
    //CHECK-NEXT:- fb_internal_dword: 0
    //CHECK-NEXT:- fb_internal_lword: 0
    //CHECK-NEXT:- fb_internal_bool: 1
    //CHECK-NEXT:- fb_internal_time: 0
    //CHECK-NEXT:- fb_internal_ltime: 1000000000

    //CHECK-NEXT:- fb_private_sint: 0
    //CHECK-NEXT:- fb_private_int: 21
    //CHECK-NEXT:- fb_private_dint: 0
    //CHECK-NEXT:- fb_private_lint: 0
    //CHECK-NEXT:- fb_private_usint: 0
    //CHECK-NEXT:- fb_private_uint: 22
    //CHECK-NEXT:- fb_private_udint: 0
    //CHECK-NEXT:- fb_private_ulint: 0
    //CHECK-NEXT:- fb_private_word: 0
    //CHECK-NEXT:- fb_private_dword: 23
    //CHECK-NEXT:- fb_private_lword: 0
    //CHECK-NEXT:- fb_private_bool: 0
    //CHECK-NEXT:- fb_private_time: 0
    //CHECK-NEXT:- fb_private_ltime: 0

    //CHECK-NEXT:- fb_constant_sint: 1
    //CHECK-NEXT:- fb_constant_int: 2
    //CHECK-NEXT:- fb_constant_dint: 3
    //CHECK-NEXT:- fb_constant_lint: 4
    //CHECK-NEXT:- fb_constant_usint: 5
    //CHECK-NEXT:- fb_constant_uint: 6
    //CHECK-NEXT:- fb_constant_udint: 7
    //CHECK-NEXT:- fb_constant_ulint: 8
    //CHECK-NEXT:- fb_constant_word: 9
    //CHECK-NEXT:- fb_constant_dword: 10
    //CHECK-NEXT:- fb_constant_lword: 11
    //CHECK-NEXT:- fb_constant_bool: 1
    //CHECK-NEXT:- fb_constant_time: 13000000
    //CHECK-NEXT:- fb_constant_ltime: 14000

    //CHECK-NEXT:Checking method variable after call:
    //CHECK-NEXT:- m_input_sint: 0
    //CHECK-NEXT:- m_input_int: 2
    //CHECK-NEXT:- m_input_dint: 0
    //CHECK-NEXT:- m_input_lint: 0
    //CHECK-NEXT:- m_input_usint: 0
    //CHECK-NEXT:- m_input_uint: 6
    //CHECK-NEXT:- m_input_udint: 0
    //CHECK-NEXT:- m_input_ulint: 0
    //CHECK-NEXT:- m_input_word: 0
    //CHECK-NEXT:- m_input_dword: 10
    //CHECK-NEXT:- m_input_lword: 0
    //CHECK-NEXT:- m_input_bool: 0
    //CHECK-NEXT:- m_input_time: 12000000000
    //CHECK-NEXT:- m_input_ltime: 0

    //CHECK-NEXT:- m_output_sint: 0
    //CHECK-NEXT:- m_output_int: 0
    //CHECK-NEXT:- m_output_dint: 3
    //CHECK-NEXT:- m_output_lint: 0
    //CHECK-NEXT:- m_output_usint: 0
    //CHECK-NEXT:- m_output_uint: 0
    //CHECK-NEXT:- m_output_udint: 0
    //CHECK-NEXT:- m_output_ulint: 8
    //CHECK-NEXT:- m_output_word: 0
    //CHECK-NEXT:- m_output_dword: 0
    //CHECK-NEXT:- m_output_lword: 11
    //CHECK-NEXT:- m_output_bool: 1
    //CHECK-NEXT:- m_output_time: 0
    //CHECK-NEXT:- m_output_ltime: 13000

    //CHECK-NEXT:- m_var_sint: 1
    //CHECK-NEXT:- m_var_int: 0
    //CHECK-NEXT:- m_var_dint: 0
    //CHECK-NEXT:- m_var_lint: 0
    //CHECK-NEXT:- m_var_usint: 5
    //CHECK-NEXT:- m_var_uint: 0
    //CHECK-NEXT:- m_var_udint: 0
    //CHECK-NEXT:- m_var_ulint: 0
    //CHECK-NEXT:- m_var_word: 9
    //CHECK-NEXT:- m_var_dword: 0
    //CHECK-NEXT:- m_var_lword: 0
    //CHECK-NEXT:- m_var_bool: 1
    //CHECK-NEXT:- m_var_time: 12000000
    //CHECK-NEXT:- m_var_ltime: 13

    fb_instance.myMethod(61,62,63,64,65,66,67,68,69,70,71,TRUE,t#73ms,lt#74ns,var_sint,var_int,var_dint,var_lint,var_usint,var_uint,var_udint,var_ulint,var_word,var_dword,var_lword,var_bool,var_time,var_ltime);

    //CHECK-NEXT:Checking fb variable after call of method:
    //CHECK-NEXT:- fb_input_sint: 0
    //CHECK-NEXT:- fb_input_int: 0
    //CHECK-NEXT:- fb_input_dint: 0
    //CHECK-NEXT:- fb_input_lint: 4
    //CHECK-NEXT:- fb_input_usint: 0
    //CHECK-NEXT:- fb_input_uint: 0
    //CHECK-NEXT:- fb_input_udint: 7
    //CHECK-NEXT:- fb_input_ulint: 0
    //CHECK-NEXT:- fb_input_word: 9
    //CHECK-NEXT:- fb_input_dword: 0
    //CHECK-NEXT:- fb_input_lword: 0
    //CHECK-NEXT:- fb_input_bool: 1
    //CHECK-NEXT:- fb_input_time: 0
    //CHECK-NEXT:- fb_input_ltime: 13000000000

    //CHECK-NEXT:- fb_output_sint: -1
    //CHECK-NEXT:- fb_output_int: -2
    //CHECK-NEXT:- fb_output_dint: -3
    //CHECK-NEXT:- fb_output_lint: -4
    //CHECK-NEXT:- fb_output_usint: 1
    //CHECK-NEXT:- fb_output_uint: 2
    //CHECK-NEXT:- fb_output_udint: 3
    //CHECK-NEXT:- fb_output_ulint: 4
    //CHECK-NEXT:- fb_output_word: 1
    //CHECK-NEXT:- fb_output_dword: 2
    //CHECK-NEXT:- fb_output_lword: 3
    //CHECK-NEXT:- fb_output_bool: 1
    //CHECK-NEXT:- fb_output_time: 1000000
    //CHECK-NEXT:- fb_output_ltime: 1

    //CHECK-NEXT:- fb_public_sint: -1
    //CHECK-NEXT:- fb_public_int: -2
    //CHECK-NEXT:- fb_public_dint: -3
    //CHECK-NEXT:- fb_public_lint: -4
    //CHECK-NEXT:- fb_public_usint: 1
    //CHECK-NEXT:- fb_public_uint: 2
    //CHECK-NEXT:- fb_public_udint: 3
    //CHECK-NEXT:- fb_public_ulint: 4
    //CHECK-NEXT:- fb_public_word: 1
    //CHECK-NEXT:- fb_public_dword: 2
    //CHECK-NEXT:- fb_public_lword: 3
    //CHECK-NEXT:- fb_public_bool: 1
    //CHECK-NEXT:- fb_public_time: 1000000
    //CHECK-NEXT:- fb_public_ltime: 1

    //CHECK-NEXT:- fb_protected_sint: -1
    //CHECK-NEXT:- fb_protected_int: -2
    //CHECK-NEXT:- fb_protected_dint: -3
    //CHECK-NEXT:- fb_protected_lint: -4
    //CHECK-NEXT:- fb_protected_usint: 1
    //CHECK-NEXT:- fb_protected_uint: 2
    //CHECK-NEXT:- fb_protected_udint: 3
    //CHECK-NEXT:- fb_protected_ulint: 4
    //CHECK-NEXT:- fb_protected_word: 1
    //CHECK-NEXT:- fb_protected_dword: 2
    //CHECK-NEXT:- fb_protected_lword: 3
    //CHECK-NEXT:- fb_protected_bool: 1
    //CHECK-NEXT:- fb_protected_time: 1000000
    //CHECK-NEXT:- fb_protected_ltime: 1

    //CHECK-NEXT:- fb_internal_sint: -1
    //CHECK-NEXT:- fb_internal_int: -2
    //CHECK-NEXT:- fb_internal_dint: -3
    //CHECK-NEXT:- fb_internal_lint: -4
    //CHECK-NEXT:- fb_internal_usint: 1
    //CHECK-NEXT:- fb_internal_uint: 2
    //CHECK-NEXT:- fb_internal_udint: 3
    //CHECK-NEXT:- fb_internal_ulint: 4
    //CHECK-NEXT:- fb_internal_word: 1
    //CHECK-NEXT:- fb_internal_dword: 2
    //CHECK-NEXT:- fb_internal_lword: 3
    //CHECK-NEXT:- fb_internal_bool: 1
    //CHECK-NEXT:- fb_internal_time: 1000000
    //CHECK-NEXT:- fb_internal_ltime: 1

    //CHECK-NEXT:- fb_private_sint: -1
    //CHECK-NEXT:- fb_private_int: -2
    //CHECK-NEXT:- fb_private_dint: -3
    //CHECK-NEXT:- fb_private_lint: -4
    //CHECK-NEXT:- fb_private_usint: 1
    //CHECK-NEXT:- fb_private_uint: 2
    //CHECK-NEXT:- fb_private_udint: 3
    //CHECK-NEXT:- fb_private_ulint: 4
    //CHECK-NEXT:- fb_private_word: 1
    //CHECK-NEXT:- fb_private_dword: 2
    //CHECK-NEXT:- fb_private_lword: 3
    //CHECK-NEXT:- fb_private_bool: 1
    //CHECK-NEXT:- fb_private_time: 1000000
    //CHECK-NEXT:- fb_private_ltime: 1

    //CHECK-NEXT:- fb_constant_sint: 1
    //CHECK-NEXT:- fb_constant_int: 2
    //CHECK-NEXT:- fb_constant_dint: 3
    //CHECK-NEXT:- fb_constant_lint: 4
    //CHECK-NEXT:- fb_constant_usint: 5
    //CHECK-NEXT:- fb_constant_uint: 6
    //CHECK-NEXT:- fb_constant_udint: 7
    //CHECK-NEXT:- fb_constant_ulint: 8
    //CHECK-NEXT:- fb_constant_word: 9
    //CHECK-NEXT:- fb_constant_dword: 10
    //CHECK-NEXT:- fb_constant_lword: 11
    //CHECK-NEXT:- fb_constant_bool: 1
    //CHECK-NEXT:- fb_constant_time: 13000000
    //CHECK-NEXT:- fb_constant_ltime: 14000

    //CHECK-NEXT:Checking method variable after call:
    //CHECK-NEXT:- m_input_sint: 61
    //CHECK-NEXT:- m_input_int: 62
    //CHECK-NEXT:- m_input_dint: 63
    //CHECK-NEXT:- m_input_lint: 64
    //CHECK-NEXT:- m_input_usint: 65
    //CHECK-NEXT:- m_input_uint: 66
    //CHECK-NEXT:- m_input_udint: 67
    //CHECK-NEXT:- m_input_ulint: 68
    //CHECK-NEXT:- m_input_word: 69
    //CHECK-NEXT:- m_input_dword: 70
    //CHECK-NEXT:- m_input_lword: 71
    //CHECK-NEXT:- m_input_bool: 1
    //CHECK-NEXT:- m_input_time: 73000000
    //CHECK-NEXT:- m_input_ltime: 74

    //CHECK-NEXT:- m_output_sint: 0
    //CHECK-NEXT:- m_output_int: 0
    //CHECK-NEXT:- m_output_dint: 3
    //CHECK-NEXT:- m_output_lint: 0
    //CHECK-NEXT:- m_output_usint: 0
    //CHECK-NEXT:- m_output_uint: 0
    //CHECK-NEXT:- m_output_udint: 0
    //CHECK-NEXT:- m_output_ulint: 8
    //CHECK-NEXT:- m_output_word: 0
    //CHECK-NEXT:- m_output_dword: 0
    //CHECK-NEXT:- m_output_lword: 11
    //CHECK-NEXT:- m_output_bool: 1
    //CHECK-NEXT:- m_output_time: 0
    //CHECK-NEXT:- m_output_ltime: 13000

    //CHECK-NEXT:- m_var_sint: 1
    //CHECK-NEXT:- m_var_int: 0
    //CHECK-NEXT:- m_var_dint: 0
    //CHECK-NEXT:- m_var_lint: 0
    //CHECK-NEXT:- m_var_usint: 5
    //CHECK-NEXT:- m_var_uint: 0
    //CHECK-NEXT:- m_var_udint: 0
    //CHECK-NEXT:- m_var_ulint: 0
    //CHECK-NEXT:- m_var_word: 9
    //CHECK-NEXT:- m_var_dword: 0
    //CHECK-NEXT:- m_var_lword: 0
    //CHECK-NEXT:- m_var_bool: 1
    //CHECK-NEXT:- m_var_time: 12000000
    //CHECK-NEXT:- m_var_ltime: 13

    PRINTF('Checking changes of variable:$N');
    PRINTF('- var_sint: %d$N', var_sint);
    PRINTF('- var_int: %d$N', var_int);
    PRINTF('- var_dint: %d$N', var_dint);
    PRINTF('- var_lint: %d$N', var_lint);
    PRINTF('- var_usint: %d$N', var_usint);
    PRINTF('- var_uint: %d$N', var_uint);
    PRINTF('- var_udint: %d$N', var_udint);
    PRINTF('- var_ulint: %d$N', var_ulint);
    PRINTF('- var_word: %d$N', var_word);
    PRINTF('- var_dword: %d$N', var_dword);
    PRINTF('- var_lword: %d$N', var_lword);
    PRINTF('- var_bool: %d$N', var_bool);
    PRINTF('- var_time: %lld$N', var_time);
    PRINTF('- var_ltime: %d$N', var_ltime);

    //CHECK-NEXT:Checking changes of variable:
    //CHECK-NEXT:- var_sint: 61
    //CHECK-NEXT:- var_int: 62
    //CHECK-NEXT:- var_dint: 63
    //CHECK-NEXT:- var_lint: 64
    //CHECK-NEXT:- var_usint: 65
    //CHECK-NEXT:- var_uint: 66
    //CHECK-NEXT:- var_udint: 67
    //CHECK-NEXT:- var_ulint: 68
    //CHECK-NEXT:- var_word: 69
    //CHECK-NEXT:- var_dword: 70
    //CHECK-NEXT:- var_lword: 71
    //CHECK-NEXT:- var_bool: 1
    //CHECK-NEXT:- var_time: 73000000
    //CHECK-NEXT:- var_ltime: 74

    fb2_instance.myMethod2();
    //CHECK-NEXT:Checking fb variable after call of method2:
    //CHECK-NEXT:- fb_input_sint: 0
    //CHECK-NEXT:- fb_input_int: 0
    //CHECK-NEXT:- fb_input_dint: 0
    //CHECK-NEXT:- fb_input_lint: 4
    //CHECK-NEXT:- fb_input_usint: 0
    //CHECK-NEXT:- fb_input_uint: 0
    //CHECK-NEXT:- fb_input_udint: 7
    //CHECK-NEXT:- fb_input_ulint: 0
    //CHECK-NEXT:- fb_input_word: 9
    //CHECK-NEXT:- fb_input_dword: 0
    //CHECK-NEXT:- fb_input_lword: 0
    //CHECK-NEXT:- fb_input_bool: 1
    //CHECK-NEXT:- fb_input_time: 0
    //CHECK-NEXT:- fb_input_ltime: 13000000000

    //CHECK-NEXT:- fb_output_sint: 1
    //CHECK-NEXT:- fb_output_int: 0
    //CHECK-NEXT:- fb_output_dint: 0
    //CHECK-NEXT:- fb_output_lint: 0
    //CHECK-NEXT:- fb_output_usint: 0
    //CHECK-NEXT:- fb_output_uint: 6
    //CHECK-NEXT:- fb_output_udint: 0
    //CHECK-NEXT:- fb_output_ulint: 0
    //CHECK-NEXT:- fb_output_word: 0
    //CHECK-NEXT:- fb_output_dword: 0
    //CHECK-NEXT:- fb_output_lword: 11
    //CHECK-NEXT:- fb_output_bool: 0
    //CHECK-NEXT:- fb_output_time: 12000000000
    //CHECK-NEXT:- fb_output_ltime: 0

    //CHECK-NEXT:- fb_public_sint: 0
    //CHECK-NEXT:- fb_public_int: 2
    //CHECK-NEXT:- fb_public_dint: 0
    //CHECK-NEXT:- fb_public_lint: 0
    //CHECK-NEXT:- fb_public_usint: 5
    //CHECK-NEXT:- fb_public_uint: 0
    //CHECK-NEXT:- fb_public_udint: 0
    //CHECK-NEXT:- fb_public_ulint: 0
    //CHECK-NEXT:- fb_public_word: 0
    //CHECK-NEXT:- fb_public_dword: 10
    //CHECK-NEXT:- fb_public_lword: 0
    //CHECK-NEXT:- fb_public_bool: 1
    //CHECK-NEXT:- fb_public_time: 0
    //CHECK-NEXT:- fb_public_ltime: 0

    //CHECK-NEXT:- fb_protected_sint: 0
    //CHECK-NEXT:- fb_protected_int: 0
    //CHECK-NEXT:- fb_protected_dint: 3
    //CHECK-NEXT:- fb_protected_lint: 0
    //CHECK-NEXT:- fb_protected_usint: 0
    //CHECK-NEXT:- fb_protected_uint: 0
    //CHECK-NEXT:- fb_protected_udint: 0
    //CHECK-NEXT:- fb_protected_ulint: 8
    //CHECK-NEXT:- fb_protected_word: 0
    //CHECK-NEXT:- fb_protected_dword: 0
    //CHECK-NEXT:- fb_protected_lword: 0
    //CHECK-NEXT:- fb_protected_bool: 0
    //CHECK-NEXT:- fb_protected_time: 42000000
    //CHECK-NEXT:- fb_protected_ltime: 0

    //CHECK-NEXT:- fb_internal_sint: 11
    //CHECK-NEXT:- fb_internal_int: 0
    //CHECK-NEXT:- fb_internal_dint: 0
    //CHECK-NEXT:- fb_internal_lint: 0
    //CHECK-NEXT:- fb_internal_usint: 12
    //CHECK-NEXT:- fb_internal_uint: 0
    //CHECK-NEXT:- fb_internal_udint: 0
    //CHECK-NEXT:- fb_internal_ulint: 0
    //CHECK-NEXT:- fb_internal_word: 13
    //CHECK-NEXT:- fb_internal_dword: 0
    //CHECK-NEXT:- fb_internal_lword: 0
    //CHECK-NEXT:- fb_internal_bool: 1
    //CHECK-NEXT:- fb_internal_time: 0
    //CHECK-NEXT:- fb_internal_ltime: 1000000000

    //CHECK-NEXT:- fb_constant_sint: 1
    //CHECK-NEXT:- fb_constant_int: 2
    //CHECK-NEXT:- fb_constant_dint: 3
    //CHECK-NEXT:- fb_constant_lint: 4
    //CHECK-NEXT:- fb_constant_usint: 5
    //CHECK-NEXT:- fb_constant_uint: 6
    //CHECK-NEXT:- fb_constant_udint: 7
    //CHECK-NEXT:- fb_constant_ulint: 8
    //CHECK-NEXT:- fb_constant_word: 9
    //CHECK-NEXT:- fb_constant_dword: 10
    //CHECK-NEXT:- fb_constant_lword: 11
    //CHECK-NEXT:- fb_constant_bool: 1
    //CHECK-NEXT:- fb_constant_time: 13000000
    //CHECK-NEXT:- fb_constant_ltime: 14000

    //CHECK-NEXT:Checking method2 variable after call:
    //CHECK-NEXT:- m_input_sint: 0
    //CHECK-NEXT:- m_input_int: 2
    //CHECK-NEXT:- m_input_dint: 0
    //CHECK-NEXT:- m_input_lint: 0
    //CHECK-NEXT:- m_input_usint: 0
    //CHECK-NEXT:- m_input_uint: 6
    //CHECK-NEXT:- m_input_udint: 0
    //CHECK-NEXT:- m_input_ulint: 0
    //CHECK-NEXT:- m_input_word: 0
    //CHECK-NEXT:- m_input_dword: 10
    //CHECK-NEXT:- m_input_lword: 0
    //CHECK-NEXT:- m_input_bool: 0
    //CHECK-NEXT:- m_input_time: 12000000000
    //CHECK-NEXT:- m_input_ltime: 0

    //CHECK-NEXT:- m_output_sint: 0
    //CHECK-NEXT:- m_output_int: 0
    //CHECK-NEXT:- m_output_dint: 3
    //CHECK-NEXT:- m_output_lint: 0
    //CHECK-NEXT:- m_output_usint: 0
    //CHECK-NEXT:- m_output_uint: 0
    //CHECK-NEXT:- m_output_udint: 0
    //CHECK-NEXT:- m_output_ulint: 8
    //CHECK-NEXT:- m_output_word: 0
    //CHECK-NEXT:- m_output_dword: 0
    //CHECK-NEXT:- m_output_lword: 11
    //CHECK-NEXT:- m_output_bool: 1
    //CHECK-NEXT:- m_output_time: 0
    //CHECK-NEXT:- m_output_ltime: 13000

    //CHECK-NEXT:- m_var_sint: 1
    //CHECK-NEXT:- m_var_int: 0
    //CHECK-NEXT:- m_var_dint: 0
    //CHECK-NEXT:- m_var_lint: 0
    //CHECK-NEXT:- m_var_usint: 5
    //CHECK-NEXT:- m_var_uint: 0
    //CHECK-NEXT:- m_var_udint: 0
    //CHECK-NEXT:- m_var_ulint: 0
    //CHECK-NEXT:- m_var_word: 9
    //CHECK-NEXT:- m_var_dword: 0
    //CHECK-NEXT:- m_var_lword: 0
    //CHECK-NEXT:- m_var_bool: 1
    //CHECK-NEXT:- m_var_time: 12000000
    //CHECK-NEXT:- m_var_ltime: 13

    fb2_instance.myMethod2(91,92,93,94,95,96,97,98,99,100,101,TRUE,t#103ms,lt#104ns,var_sint,var_int,var_dint,var_lint,var_usint,var_uint,var_udint,var_ulint,var_word,var_dword,var_lword,var_bool,var_time,var_ltime);

    //CHECK-NEXT:Checking fb variable after call of method2:
    //CHECK-NEXT:- fb_input_sint: 0
    //CHECK-NEXT:- fb_input_int: 0
    //CHECK-NEXT:- fb_input_dint: 0
    //CHECK-NEXT:- fb_input_lint: 4
    //CHECK-NEXT:- fb_input_usint: 0
    //CHECK-NEXT:- fb_input_uint: 0
    //CHECK-NEXT:- fb_input_udint: 7
    //CHECK-NEXT:- fb_input_ulint: 0
    //CHECK-NEXT:- fb_input_word: 9
    //CHECK-NEXT:- fb_input_dword: 0
    //CHECK-NEXT:- fb_input_lword: 0
    //CHECK-NEXT:- fb_input_bool: 1
    //CHECK-NEXT:- fb_input_time: 0
    //CHECK-NEXT:- fb_input_ltime: 13000000000

    //CHECK-NEXT:- fb_output_sint: -1
    //CHECK-NEXT:- fb_output_int: -2
    //CHECK-NEXT:- fb_output_dint: -3
    //CHECK-NEXT:- fb_output_lint: -4
    //CHECK-NEXT:- fb_output_usint: 1
    //CHECK-NEXT:- fb_output_uint: 2
    //CHECK-NEXT:- fb_output_udint: 3
    //CHECK-NEXT:- fb_output_ulint: 4
    //CHECK-NEXT:- fb_output_word: 1
    //CHECK-NEXT:- fb_output_dword: 2
    //CHECK-NEXT:- fb_output_lword: 3
    //CHECK-NEXT:- fb_output_bool: 1
    //CHECK-NEXT:- fb_output_time: 1000000
    //CHECK-NEXT:- fb_output_ltime: 1

    //CHECK-NEXT:- fb_public_sint: -1
    //CHECK-NEXT:- fb_public_int: -2
    //CHECK-NEXT:- fb_public_dint: -3
    //CHECK-NEXT:- fb_public_lint: -4
    //CHECK-NEXT:- fb_public_usint: 1
    //CHECK-NEXT:- fb_public_uint: 2
    //CHECK-NEXT:- fb_public_udint: 3
    //CHECK-NEXT:- fb_public_ulint: 4
    //CHECK-NEXT:- fb_public_word: 1
    //CHECK-NEXT:- fb_public_dword: 2
    //CHECK-NEXT:- fb_public_lword: 3
    //CHECK-NEXT:- fb_public_bool: 1
    //CHECK-NEXT:- fb_public_time: 1000000
    //CHECK-NEXT:- fb_public_ltime: 1

    //CHECK-NEXT:- fb_protected_sint: -1
    //CHECK-NEXT:- fb_protected_int: -2
    //CHECK-NEXT:- fb_protected_dint: -3
    //CHECK-NEXT:- fb_protected_lint: -4
    //CHECK-NEXT:- fb_protected_usint: 1
    //CHECK-NEXT:- fb_protected_uint: 2
    //CHECK-NEXT:- fb_protected_udint: 3
    //CHECK-NEXT:- fb_protected_ulint: 4
    //CHECK-NEXT:- fb_protected_word: 1
    //CHECK-NEXT:- fb_protected_dword: 2
    //CHECK-NEXT:- fb_protected_lword: 3
    //CHECK-NEXT:- fb_protected_bool: 1
    //CHECK-NEXT:- fb_protected_time: 1000000
    //CHECK-NEXT:- fb_protected_ltime: 1

    //CHECK-NEXT:- fb_internal_sint: -1
    //CHECK-NEXT:- fb_internal_int: -2
    //CHECK-NEXT:- fb_internal_dint: -3
    //CHECK-NEXT:- fb_internal_lint: -4
    //CHECK-NEXT:- fb_internal_usint: 1
    //CHECK-NEXT:- fb_internal_uint: 2
    //CHECK-NEXT:- fb_internal_udint: 3
    //CHECK-NEXT:- fb_internal_ulint: 4
    //CHECK-NEXT:- fb_internal_word: 1
    //CHECK-NEXT:- fb_internal_dword: 2
    //CHECK-NEXT:- fb_internal_lword: 3
    //CHECK-NEXT:- fb_internal_bool: 1
    //CHECK-NEXT:- fb_internal_time: 1000000
    //CHECK-NEXT:- fb_internal_ltime: 1

    //CHECK-NEXT:- fb_constant_sint: 1
    //CHECK-NEXT:- fb_constant_int: 2
    //CHECK-NEXT:- fb_constant_dint: 3
    //CHECK-NEXT:- fb_constant_lint: 4
    //CHECK-NEXT:- fb_constant_usint: 5
    //CHECK-NEXT:- fb_constant_uint: 6
    //CHECK-NEXT:- fb_constant_udint: 7
    //CHECK-NEXT:- fb_constant_ulint: 8
    //CHECK-NEXT:- fb_constant_word: 9
    //CHECK-NEXT:- fb_constant_dword: 10
    //CHECK-NEXT:- fb_constant_lword: 11
    //CHECK-NEXT:- fb_constant_bool: 1
    //CHECK-NEXT:- fb_constant_time: 13000000
    //CHECK-NEXT:- fb_constant_ltime: 14000

    //CHECK-NEXT:Checking method2 variable after call:
    //CHECK-NEXT:- m_input_sint: 91
    //CHECK-NEXT:- m_input_int: 92
    //CHECK-NEXT:- m_input_dint: 93
    //CHECK-NEXT:- m_input_lint: 94
    //CHECK-NEXT:- m_input_usint: 95
    //CHECK-NEXT:- m_input_uint: 96
    //CHECK-NEXT:- m_input_udint: 97
    //CHECK-NEXT:- m_input_ulint: 98
    //CHECK-NEXT:- m_input_word: 99
    //CHECK-NEXT:- m_input_dword: 100
    //CHECK-NEXT:- m_input_lword: 101
    //CHECK-NEXT:- m_input_bool: 1
    //CHECK-NEXT:- m_input_time: 103000000
    //CHECK-NEXT:- m_input_ltime: 104

    //CHECK-NEXT:- m_output_sint: 0
    //CHECK-NEXT:- m_output_int: 0
    //CHECK-NEXT:- m_output_dint: 3
    //CHECK-NEXT:- m_output_lint: 0
    //CHECK-NEXT:- m_output_usint: 0
    //CHECK-NEXT:- m_output_uint: 0
    //CHECK-NEXT:- m_output_udint: 0
    //CHECK-NEXT:- m_output_ulint: 8
    //CHECK-NEXT:- m_output_word: 0
    //CHECK-NEXT:- m_output_dword: 0
    //CHECK-NEXT:- m_output_lword: 11
    //CHECK-NEXT:- m_output_bool: 1
    //CHECK-NEXT:- m_output_time: 0
    //CHECK-NEXT:- m_output_ltime: 13000

    //CHECK-NEXT:- m_var_sint: 1
    //CHECK-NEXT:- m_var_int: 0
    //CHECK-NEXT:- m_var_dint: 0
    //CHECK-NEXT:- m_var_lint: 0
    //CHECK-NEXT:- m_var_usint: 5
    //CHECK-NEXT:- m_var_uint: 0
    //CHECK-NEXT:- m_var_udint: 0
    //CHECK-NEXT:- m_var_ulint: 0
    //CHECK-NEXT:- m_var_word: 9
    //CHECK-NEXT:- m_var_dword: 0
    //CHECK-NEXT:- m_var_lword: 0
    //CHECK-NEXT:- m_var_bool: 1
    //CHECK-NEXT:- m_var_time: 12000000
    //CHECK-NEXT:- m_var_ltime: 13

    PRINTF('Checking changes of variable:$N');
    PRINTF('- var_sint: %d$N', var_sint);
    PRINTF('- var_int: %d$N', var_int);
    PRINTF('- var_dint: %d$N', var_dint);
    PRINTF('- var_lint: %d$N', var_lint);
    PRINTF('- var_usint: %d$N', var_usint);
    PRINTF('- var_uint: %d$N', var_uint);
    PRINTF('- var_udint: %d$N', var_udint);
    PRINTF('- var_ulint: %d$N', var_ulint);
    PRINTF('- var_word: %d$N', var_word);
    PRINTF('- var_dword: %d$N', var_dword);
    PRINTF('- var_lword: %d$N', var_lword);
    PRINTF('- var_bool: %d$N', var_bool);
    PRINTF('- var_time: %lld$N', var_time);
    PRINTF('- var_ltime: %d$N', var_ltime);

    //CHECK-NEXT:Checking changes of variable:
    //CHECK-NEXT:- var_sint: 91
    //CHECK-NEXT:- var_int: 92
    //CHECK-NEXT:- var_dint: 93
    //CHECK-NEXT:- var_lint: 94
    //CHECK-NEXT:- var_usint: 95
    //CHECK-NEXT:- var_uint: 96
    //CHECK-NEXT:- var_udint: 97
    //CHECK-NEXT:- var_ulint: 98
    //CHECK-NEXT:- var_word: 99
    //CHECK-NEXT:- var_dword: 100
    //CHECK-NEXT:- var_lword: 101
    //CHECK-NEXT:- var_bool: 1
    //CHECK-NEXT:- var_time: 103000000
    //CHECK-NEXT:- var_ltime: 104

    PRINTF('Return value of myMethod_sint: %hhd$N', fb2_instance.myMethod_sint(var_sint));
    //CHECK-NEXT:Return value of myMethod_sint: 91
    PRINTF('Return value of myMethod_int: %hd$N', fb2_instance.myMethod_int(var_int));
    //CHECK-NEXT:Return value of myMethod_int: 92
    PRINTF('Return value of myMethod_dint: %d$N', fb2_instance.myMethod_dint(var_dint));
    //CHECK-NEXT:Return value of myMethod_dint: 93
    PRINTF('Return value of myMethod_lint: %lld$N', fb2_instance.myMethod_lint(var_lint));
    //CHECK-NEXT:Return value of myMethod_lint: 94
    PRINTF('Return value of myMethod_usint: %hhu$N', fb2_instance.myMethod_usint(var_usint));
    //CHECK-NEXT:Return value of myMethod_usint: 95
    PRINTF('Return value of myMethod_uint: %hu$N', fb2_instance.myMethod_uint(var_uint));
    //CHECK-NEXT:Return value of myMethod_uint: 96
    PRINTF('Return value of myMethod_udint: %u$N', fb2_instance.myMethod_udint(var_udint));
    //CHECK-NEXT:Return value of myMethod_udint: 97
    PRINTF('Return value of myMethod_ulint: %llu$N', fb2_instance.myMethod_ulint(var_ulint));
    //CHECK-NEXT:Return value of myMethod_ulint: 98
    PRINTF('Return value of myMethod_word: %hu$N', fb2_instance.myMethod_word(var_word));
    //CHECK-NEXT:Return value of myMethod_word: 99
    PRINTF('Return value of myMethod_dword: %u$N', fb2_instance.myMethod_dword(var_dword));
    //CHECK-NEXT:Return value of myMethod_dword: 100
    PRINTF('Return value of myMethod_lword: %llu$N', fb2_instance.myMethod_lword(var_lword));
    //CHECK-NEXT:Return value of myMethod_lword: 101
    PRINTF('Return value of myMethod_bool: %d$N', fb2_instance.myMethod_bool(var_bool));
    //CHECK-NEXT:Return value of myMethod_bool: 1
    PRINTF('Return value of myMethod_time: %lld$N', fb2_instance.myMethod_time(var_time));
    //CHECK-NEXT:Return value of myMethod_time: 103000000
    PRINTF('Return value of myMethod_ltime: %lld$N', fb2_instance.myMethod_ltime(var_ltime));
    //CHECK-NEXT:Return value of myMethod_ltime: 104
    var_arr_int := fb2_instance.myMethod_arr_int(1,3);
    PRINTF('Return value of myMethod_arr_int[1]: %hd, myMethod_arr_int[2]: %hd, myMethod_arr_int[3]: %hd, myMethod_arr_int[4]: %hd, myMethod_arr_int[5]: %hd$N', var_arr_int[1], var_arr_int[2], var_arr_int[3], var_arr_int[4], var_arr_int[5]);
    //CHECK-NEXT:Return value of myMethod_arr_int[1]: 1, myMethod_arr_int[2]: 0, myMethod_arr_int[3]: 3, myMethod_arr_int[4]: 0, myMethod_arr_int[5]: 0
    fb_instance.myMethod_bits(23, var_bits);
    PRINTF('Return value of myMethod_byte: %d, %d, %d, %d, %d, %d, %d, %d$N', var_bits[1], var_bits[2], var_bits[3], var_bits[4], var_bits[5], var_bits[6], var_bits[7], var_bits[8]);
    //CHECK-NEXT:Return value of myMethod_byte: 1, 1, 1, 0, 1, 0, 0, 0

    if (fb2_instance.myMethod_bool(var_bool)) THEN
        PRINTF('Return value of myMethod_bool inside IF: %d$N', fb2_instance.myMethod_bool(var_bool));
    END_IF
    //CHECK-NEXT:Return value of myMethod_bool inside IF: 1

    if (fb2_instance.myMethod_int(1) = fb_instance.myMethod_int(1)) THEN
        PRINTF('Return value of two methods from different instances are compared successfully with each other. %hd = %hd$N', fb2_instance.myMethod_int(1), fb_instance.myMethod_int(1));
    END_IF
    //CHECK-NEXT:Return value of two methods from different instances are compared successfully with each other. 1 = 1

    if (fb2_instance.myMethod_usint(3) = 0) THEN
        ;
    ELSIF (fb_instance.myMethod_udint(fb2_instance.myMethod_udint(3)) = 3) THEN
        PRINTF('Return value of myMethod_udint inside ELSIF: %u$N', fb_instance.myMethod_udint(fb2_instance.myMethod_udint(3)));
    END_IF
    //CHECK-NEXT:Return value of myMethod_udint inside ELSIF: 3

    CASE fb2_instance.myMethod_lint(fb_instance.myMethod_lint(32)) OF
        0:;
        32:
            PRINTF('Return value of myMethod_lint inside CASE: %lld$N', fb2_instance.myMethod_lint(32));
        1:;
    END_CASE
    //CHECK-NEXT:Return value of myMethod_lint inside CASE: 32

    FOR var_int := 1 TO 10 DO
        PRINTF('Return value of var_int inside FOR loop: %hd$N', fb_instance.myMethod_int(var_int));
    END_FOR
    //CHECK-NEXT:Return value of var_int inside FOR loop: 1
    //CHECK-NEXT:Return value of var_int inside FOR loop: 2
    //CHECK-NEXT:Return value of var_int inside FOR loop: 3
    //CHECK-NEXT:Return value of var_int inside FOR loop: 4
    //CHECK-NEXT:Return value of var_int inside FOR loop: 5
    //CHECK-NEXT:Return value of var_int inside FOR loop: 6
    //CHECK-NEXT:Return value of var_int inside FOR loop: 7
    //CHECK-NEXT:Return value of var_int inside FOR loop: 8
    //CHECK-NEXT:Return value of var_int inside FOR loop: 9
    //CHECK-NEXT:Return value of var_int inside FOR loop: 10

    var_lint := 0;
    REPEAT
        var_lint := fb_instance.myMethod_lint(var_lint + 1);
        PRINTF('Return value of var_lint inside REPEAT loop: %lld$N', var_lint);
    UNTIL var_lint >= fb2_instance.myMethod_lint(10)
    END_REPEAT
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 1
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 2
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 3
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 4
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 5
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 6
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 7
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 8
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 9
    //CHECK-NEXT:Return value of var_lint inside REPEAT loop: 10

    WHILE var_lint < fb2_instance.myMethod_lint(20) DO
        var_lint := fb_instance.myMethod_lint(var_lint + 1);
        PRINTF('Return value of var_lint inside WHILE loop: %lld$N', var_lint);
    END_WHILE
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 11
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 12
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 13
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 14
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 15
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 16
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 17
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 18
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 19
    //CHECK-NEXT:Return value of var_lint inside WHILE loop: 20
END_FUNCTION

// UNSUPPORTED: rusty
// error: failed test with no error information
// PRG-4979