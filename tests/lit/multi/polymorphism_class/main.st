FUNCTION main
    VAR
        instanceA: ClassA;
        instanceB: ClassB;
        instanceC: ClassC;
        instanceD: ClassD;

        refInstanceA: POINTER TO ClassA;
    END_VAR

    refInstanceA := ADR(instanceA);
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();

    refInstanceA := ADR(instanceB);
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();
    
    refInstanceA := ADR(instanceC);
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();

    refInstanceA := ADR(instanceD);
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();
END_FUNCTION