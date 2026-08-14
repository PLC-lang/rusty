// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// FB retain members with inline type definitions keep their in-place layout; the
// instances get the retain treatment transitively: the program instance is extracted
// into a retain global, the plain global instance moves to a retain block.
FUNCTION_BLOCK Fb
    VAR RETAIN
        arr : ARRAY[0..2] OF DINT := [1, 2, 3];
        sub : DINT(0..100) := 50;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM mainProg
    VAR
        inst : Fb;
    END_VAR

    inst.arr[0] := inst.arr[0] + 1;
    inst.sub := inst.sub + 1;
END_PROGRAM

VAR_GLOBAL
    gInst : Fb;
END_VAR

FUNCTION main : DINT
    mainProg();
    gInst.arr[2] := 99;
    gInst.sub := gInst.sub + 2;
    printf('%d %d %d$N', mainProg.inst.arr[0], mainProg.inst.arr[1], mainProg.inst.arr[2]); //CHECK: 2 2 3
    printf('%d$N', mainProg.inst.sub);                                                      //CHECK-NEXT: 51
    // Implicitly created global variable backing the program instance
    printf('%d %d$N', __mainProg_inst__retain.arr[0], __mainProg_inst__retain.sub);         //CHECK-NEXT: 2 51
    printf('%d %d$N', gInst.arr[2], gInst.sub);                                             //CHECK-NEXT: 99 52
END_FUNCTION
