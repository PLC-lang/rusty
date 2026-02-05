FUNCTION main
VAR
    message1: T_Message := "One ring to rule them all,";
    message2: T_Message := "one ring to find them.";
    message3: T_Message := "One ring to bring them all,";
    message4: T_Message := "and in the darkness bind them.";
END_VAR
    // Print message 1
    PrintMessage(message1);     // CHECK: One ring to rule them all,

    // Print message 2
    PrintMessage(message2);     // CHECK: one ring to find them.

    // Print message 3
    PrintMessage(message3);     // CHECK: One ring to bring them all,

    // Print message 4
    PrintMessage(message4);     // CHECK: and in the darkness bind them.
END_FUNCTION
