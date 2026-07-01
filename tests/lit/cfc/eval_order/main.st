// Entry point: seed acc=5, run the network, and print acc. The two blocks are wired in
// reverse document order vs their evaluation priority (AddOne=0 before DoubleIt=1), so
// priority order gives (5+1)*2 = 12, whereas document order would give (5*2)+1 = 11.
FUNCTION main : DINT
    acc := 5;
    Order();
    printf('acc = %d$N', acc);
END_FUNCTION
