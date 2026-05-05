// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Anchors that a chained qualified call (`h.inst.bar()`) resolves end-to-end
// when the call's operator is a multi-level ReferenceExpr whose base is itself
// a ReferenceExpr. Counterpart to the resolver-only unit test
// `chained_qualified_call_resolves_through_recursive_bases`.
FUNCTION_BLOCK SomeFb
    METHOD bar : INT
        bar := 7;
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder : STRUCT inst : SomeFb; END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        h : Holder;
        x : INT;
    END_VAR

    x := h.inst.bar();
    printf('chained=%d$N', x);
    // CHECK: chained=7
END_FUNCTION
