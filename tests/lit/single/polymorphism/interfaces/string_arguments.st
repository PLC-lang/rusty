INTERFACE A
    METHOD foo
        VAR_INPUT
            str: STRING;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD foo
        VAR_INPUT
            str: STRING;
        END_VAR

        printf('FbA::foo; str = %s$N', str);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;

        localInput: STRING := 'Hello, world!';
    END_VAR

    reference := instance;
    reference.foo(localInput); // Works
    reference.foo('direct'); // does not work, "Cannot generate String-Literal for type ____fwd_A_foo_self at: tests/lit/single/polymorphism/interfaces/string_arguments.st:28:18:{28:18-28:26}: ."

END_FUNCTION