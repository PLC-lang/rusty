FUNCTION main
VAR
    numberContainerFb : NumberContainer;
END_VAR
    numberContainerFb();
    PrintNumber(numberContainerFb.GetMemberValue());  // CHECK: The number you asked for: 42
END_FUNCTION
