// RUN: %COMPILE %s && %RUN | %CHECK %s
// Two narrow unsigned operands are computed as UDINT, so a product beyond the DINT range stays exact.
FUNCTION main : DINT
    VAR
        ui : UINT := 65535;
        ui2 : UINT := 65535;
        w : WORD := 16#FFFF;
        us : USINT := 255;
        r : UDINT;
        l : LINT;
    END_VAR

    r := ui * ui2;
    printf('UDINT := UINT 65535 * UINT 65535 = %u$N', r);   // CHECK: UDINT := UINT 65535 * UINT 65535 = 4294836225
    l := ui * ui2;
    printf('LINT := UINT 65535 * UINT 65535 = %lld$N', l);  // CHECK-NEXT: LINT := UINT 65535 * UINT 65535 = 4294836225
    r := ui * w;
    printf('UDINT := UINT 65535 * WORD 16#FFFF = %u$N', r); // CHECK-NEXT: UDINT := UINT 65535 * WORD 16#FFFF = 4294836225
    r := ui * us;
    printf('UDINT := UINT 65535 * USINT 255 = %u$N', r);    // CHECK-NEXT: UDINT := UINT 65535 * USINT 255 = 16711425
END_FUNCTION
