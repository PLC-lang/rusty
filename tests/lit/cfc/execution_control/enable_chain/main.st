FUNCTION main : DINT
    // Both gates FALSE: no gated block ever ran, the ungated ones compute
    // from the defaults, so zeros flow through the whole chain.
    enable_chain();
    // CHECK: idle: counter = 0, result = 0
    printf('idle: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);

    // Both gates TRUE, in sync: 5 accumulates, then (5 * 2 + 1) * 2 = 22.
    enable_chain.g1 := TRUE;
    enable_chain.g2 := TRUE;
    enable_chain();
    // CHECK: sync-on: counter = 5, result = 22
    printf('sync-on: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);

    // Out of sync, head runs and tail holds: the counter advances to 10, but
    // addOne is frozen at 11, so the tail keeps producing 22.
    enable_chain.g2 := FALSE;
    enable_chain();
    // CHECK: desync-head: counter = 10, result = 22
    printf('desync-head: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);

    // Out of sync the other way, head holds and tail runs: the counter keeps
    // its buffered 10 although seed changed, and the tail recomputes
    // (10 * 2 + 1) * 2 = 42 from that buffer.
    enable_chain.g1 := FALSE;
    enable_chain.g2 := TRUE;
    enable_chain.seed := 100;
    enable_chain();
    // CHECK: desync-tail: counter = 10, result = 42
    printf('desync-tail: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);

    // Both gates FALSE again: everything holds.
    enable_chain.g2 := FALSE;
    enable_chain();
    // CHECK: sync-off: counter = 10, result = 42
    printf('sync-off: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);

    // Both gates TRUE again: the new seed lands, (110 * 2 + 1) * 2 = 442.
    enable_chain.g1 := TRUE;
    enable_chain.g2 := TRUE;
    enable_chain();
    // CHECK: resync: counter = 110, result = 442
    printf('resync: counter = %d, result = %d$N', enable_chain.c.out, enable_chain.result);
END_FUNCTION
