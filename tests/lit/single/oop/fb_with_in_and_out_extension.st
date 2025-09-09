// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
METHOD met1 : INT
    VAR_INPUT
    mandatoryInput : INT;
    optionalInput : INT := 5;
    END_VAR
    VAR_OUTPUT
    outputValue : INT;
    END_VAR
    outputValue := mandatoryInput + optionalInput;
    printf('method 1 - mandatoryInput: %d, optionalInput: %d$N', mandatoryInput, optionalInput);
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK foo2 EXTENDS foo
VAR
    x : INT;
END_VAR
met1(
	mandatoryInput := 0,
	optionalInput := 0,
	outputValue => x
);
// CHECK: method 1 - mandatoryInput: 0, optionalInput: 0
// CHECK: method output : 0
printf('method output : %d$N', x);
met1(
	mandatoryInput := 5,
	outputValue => x
);
// CHECK: method 1 - mandatoryInput: 5, optionalInput: 5
// CHECK: method output : 10
printf('method output : %d$N', x);
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    foo_inst: foo;
    foo2_inst : foo2;
    out : INT;
END_VAR
foo2_inst();

foo2.met1(mandatoryInput:= 1, outputValue => out);
// CHECK: method 1 - mandatoryInput: 1, optionalInput: 5
// CHECK: method output : 6
printf('method output : %d$N', out);

foo.met1(mandatoryInput:= 2, outputValue => out);
// CHECK: method 1 - mandatoryInput: 2, optionalInput: 5
// CHECK: method output : 7
printf('method output : %d$N', out);

END_FUNCTION
