// Old-style library: FB_A holds a `POINTER TO FB_B` and invokes the
// pointee via `inst_fb_b^()`. The library compiles in isolation — at
// this point there is no derivation of FB_B in scope, no methods, no
// interfaces. The dispatch site MUST nonetheless go through the
// vtable, because a downstream consumer may extend FB_B and retarget
// the pointer.

FUNCTION_BLOCK FB_B
    printf('FB_B body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_A
VAR
    inst_fb_b : POINTER TO FB_B;
END_VAR
    inst_fb_b^();
END_FUNCTION_BLOCK
