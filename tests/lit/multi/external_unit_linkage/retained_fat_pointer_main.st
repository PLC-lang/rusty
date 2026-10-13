FUNCTION main
    bindA();
    bindB();
    consumerA();
    consumerB();
    printf('%d %d$N', consumerA.result, consumerB.result);
END_FUNCTION
