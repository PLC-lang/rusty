// Base FB lives in its own compilation unit. The polymorphism participant
// emits a vtable type and instance for Base; the per-unit re-index has to
// make those visible to the initializer participant before it synthesizes
// the wrapper's constructor (see Phase 4.5 revert: a split of the
// polymorphism table pass into its own UnitLowerer broke 13 initializer
// snapshots because `__vtable__ctor(self.__vtable)` stopped being emitted
// for FBs whose vtables had been split off into a separate pass).

FUNCTION_BLOCK Base
METHOD PUBLIC kind : DINT
    kind := 1;
END_METHOD
END_FUNCTION_BLOCK
