INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION bind : BOOL
    VAR_INPUT
        refIn : REFERENCE TO IA;
    END_VAR

    refIn.foo();
    bind := TRUE;
END_FUNCTION

FUNCTION_BLOCK Holder
    VAR_INPUT
        enabled : BOOL;
    END_VAR
    VAR_OUTPUT
        bound : BOOL;
    END_VAR
    VAR
        instance : FbA;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        holder : Holder;
    END_VAR

    holder.rebind();
    printf('bound = %d$N', holder.bound);
    holder.enabled := TRUE;
    holder.rebind();
    printf('bound = %d$N', holder.bound);
END_FUNCTION
