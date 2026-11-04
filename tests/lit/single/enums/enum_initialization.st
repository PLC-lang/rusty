// RUN: %COMPILE %s && %RUN | %CHECK %s

TYPE
    // without initializers
    e1 : (e1, e2);                          // no element initializers, default: e1.e1=0
    e2 : (e1:=0, e2, e3);                   // 1st element initialized 0, default: e2.e1=0
    e3 : (e1:=1, e2, e3:=0);                // some elements initialized, e3 explicit 0, default: e3.e3=0
    e4 : (e1:=2, e2:=-1, e3);               // some elements initialized, e3 implicit 0, default: e4.e3=0
    e5 : (e1:=7, e2:=2, e3);                // some elements initialized, none is 0, default: e5.e1=7
    // with initializer
    e10 : (e1, e2) := e10.e1;                   // explicitely the first, default: e10.e1=0
    e11 : (e1, e2, e3) := e11.e2;               // explicitely other (not the implicit 0), default: e11.e2=1
    e12 : (e1:=1, e2, e3:=0) := e12.e2;         // explicitely other (not the explicit 0), default: e12.e2=2
    e13 : (e1:=7, e2:=3, e3:=-2) := e13.e2;     // explicitely other, default: e13.e2=3
END_TYPE
FUNCTION main
    VAR
        v1 : e1;
        v2 : e2;
        v3 : e3;
        v4 : e4;
        v5 : e5;
        v10 : e10;
        v11 : e11;
        v12 : e12;
        v13 : e13;
    END_VAR
    
    PRINTF('e1:%d$N', v1);
    // CHECK:e1:0
    PRINTF('e2:%d$N', v2);
    // CHECK:e2:0
    PRINTF('e3:%d$N', v3);
    // CHECK:e3:0
    PRINTF('e4:%d$N', v4);
    // CHECK:e4:0
    PRINTF('e5:%d$N', v5);
    // CHECK:e5:7
    PRINTF('e10:%d$N', v10);
    // CHECK:e10:0
    PRINTF('e11:%d$N', v11);
    // CHECK:e11:1
    PRINTF('e12:%d$N', v12);
    // CHECK:e12:2
    PRINTF('e13:%d$N', v13);
    // CHECK:e13:3
END_FUNCTION