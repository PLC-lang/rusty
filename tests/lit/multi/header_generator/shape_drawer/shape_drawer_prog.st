FUNCTION main
VAR
    rect: Rectangle;
    sqr: Square;
    shp: POINTER TO Shape;
END_VAR
    rect.Draw();
    sqr.DrawAction();

    // The below should start working after the init changes have been made
    // shp := ADR(rect);
    // shp^.Draw();

    // shp := ADR(sqr);
    // shp^.Draw();
END_FUNCTION
