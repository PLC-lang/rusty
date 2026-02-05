FUNCTION main
VAR
    rect: Rectangle;
    sqr: Square;
    shp: POINTER TO Shape;
END_VAR
    rect.Draw();
    // CHECK: ******
    // CHECK-NEXT: ******
    // CHECK-NEXT: ******

    sqr.DrawAction();
    // CHECK: ****
    // CHECK-NEXT: ****
    // CHECK-NEXT: ****
    // CHECK-NEXT: ****

    // The below should start working after the init changes have been made
    // shp := ADR(rect);
    // shp^.Draw();

    // shp := ADR(sqr);
    // shp^.Draw();
END_FUNCTION
