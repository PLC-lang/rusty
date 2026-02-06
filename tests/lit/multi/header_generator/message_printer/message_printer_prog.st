FUNCTION main
VAR
    message1: T_Message := "One ring to rule them all,";
    message2: T_Message := "one ring to find them.";
    message3: T_Message := "One ring to bring them all,";
    message4: T_Message := "and in the darkness bind them.";

    message5: T_Message := "Don't panic...";
    message6: T_Message := "... or forget your towel!";

    messages7: T_Message := "... a long time ago ...";
    messages8: T_Message := "... in a galaxy ...";
    messages9: T_Message := "... far far away ...";
END_VAR
    // Print message 1
    PrintMessage(message1);     // CHECK: One ring to rule them all,

    // Print message 2
    PrintMessage(message2);     // CHECK: one ring to find them.

    // Print message 3
    PrintMessage(message3);     // CHECK: One ring to bring them all,

    // Print message 4
    PrintMessage(message4);     // CHECK: and in the darkness bind them.

    // Print messages 5 & 6
    ManualMultiMessagePrinter(2, message5, message6);
    // CHECK: Don't panic...
    // CHECK-NEXT: ... or forget your towel!

    // Print messages 7, 8 & 9
    SizedMultiMessagePrinter(3, messages7, messages8, messages9);
    // CHECK: ... a long time ago ...
    // CHECK-NEXT: ... in a galaxy ...
    // CHECK-NEXT: ... far far away ...
END_FUNCTION
