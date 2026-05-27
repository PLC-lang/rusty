FUNCTION main : DINT
VAR
    labeller : Labeller;
    dispatcher : Dispatcher;
    s : STRING[16];
END_VAR

dispatcher.bind(labeller);

s := dispatcher.label_for(0);
// CHECK: label[0] = zero
printf('label[0] = %s$N', s);

s := dispatcher.label_for(7);
// CHECK: label[7] = item
printf('label[7] = %s$N', s);
END_FUNCTION
