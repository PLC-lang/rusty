// Wrapper holds a `Derived` as a member field. The initializer
// participant must synthesize a constructor for `Wrapper` that:
//   1. Initialises Wrapper's own vtable (`__Wrapper___vtable__ctor(self.__vtable)`).
//   2. Calls `Derived__ctor(self.member)` so Derived's constructor runs and
//      sets up `self.member.__vtable`.
// Both steps cross compilation-unit boundaries; the post-index re-index
// has to make both vtables visible to this unit's initializer pass.

FUNCTION_BLOCK Wrapper
VAR
    member : Derived;
END_VAR
END_FUNCTION_BLOCK
