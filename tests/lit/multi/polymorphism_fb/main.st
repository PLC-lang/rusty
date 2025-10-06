FUNCTION main
    VAR
        instanceA: FbA;
        instanceB: FbB;
        instanceC: FbC;
        instanceD: FbD;

        refInstanceA: POINTER TO FbA;
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