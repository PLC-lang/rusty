FUNCTION more_of : DINT
VAR_INPUT
    m : IMore;
END_VAR
VAR
    o : Other := (v := 3);
    other : IOther;
END_VAR
    other := o;
    more_of := m.id() + m.wide("abc") + m.take_ref(other);
END_FUNCTION
