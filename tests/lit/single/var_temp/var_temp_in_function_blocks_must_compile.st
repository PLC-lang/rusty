// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK baseControl
    VAR_INPUT
        a_in : INT;
    END_VAR

    VAR_IN_OUT
        inoutVar : BOOL;
    END_VAR

    VAR_TEMP
        tempo : INT;
    END_VAR

    VAR_OUTPUT
        a_out :INT;
    END_VAR

    VAR

    END_VAR

    a_out := a_in + 1;

END_FUNCTION_BLOCK

FUNCTION main
   VAR
       fb : baseControl;
       fb2 : baseControl;
       dipin : BOOL;
       outVar : INT;
   END_VAR

   fb(a_in := 9, a_out => outVar, inoutVar := dipin);

   // CHECK: 10
   printf('%d$N', outVar);

   fb2(12, dipin, outVar);

   // CHECK: 13
   printf('%d$N', outVar);
END_FUNCTION
