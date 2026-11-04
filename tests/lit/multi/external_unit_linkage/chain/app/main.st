FUNCTION main : DINT
VAR
    impl : Impl;
    concrete : Concrete;
    grand : GrandChild;
    abstract_shape : POINTER TO AbstractShape;
    base : POINTER TO Base;
    child : Child;
    class_base : POINTER TO CBase;
    class_child : CChild;
    prop : POINTER TO PropBase;
    prop_child : PropChild;
    name : STRING[20];
    user : User;
END_VAR
    printf('%d %d %d$N', shape_area(impl), shape_area(concrete), shape_area(grand));
    abstract_shape := REF(grand);
    printf('%d %s$N', abstract_shape^.area('ft'), REF(grand.unit_seen));
    printf('%d$N', base_of(impl));

    base := REF(child);
    printf('%d %d %d$N', base^.kind(), base^.base_id(), extra_of(child));

    class_base := REF(class_child);
    printf('%d$N', class_base^.value());

    prop := REF(prop_child);
    name := prop^.name;
    printf('%s$N', REF(name));

    user.shape := impl;
    user.shapes[0] := concrete;
    user.shapes[1] := grand;
    printf('%d$N', user.total());
END_FUNCTION
