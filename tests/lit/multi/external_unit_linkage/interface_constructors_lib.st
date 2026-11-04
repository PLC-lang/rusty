INTERFACE IText
    METHOD put : DINT
    VAR_INPUT
        s : STRING[20];
        a : ARRAY[0..3] OF DINT;
        w : WSTRING[20];
    END_VAR
    END_METHOD

    METHOD get : STRING[20] END_METHOD

    METHOD split : DINT
    VAR_OUTPUT
        head : STRING[20];
    END_VAR
    VAR_IN_OUT
        tail : STRING[20];
    END_VAR
    END_METHOD

    METHOD by_ref : DINT
    VAR_INPUT
        r : REFERENCE TO STRING[20];
        p : REF_TO DINT;
    END_VAR
    END_METHOD

    PROPERTY_GET label : STRING[20] END_PROPERTY
END_INTERFACE

INTERFACE INamed EXTENDS IText
    METHOD rename : DINT
    VAR_INPUT
        n : STRING[20];
    END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK LibText IMPLEMENTS INamed
VAR
    text : STRING[20] := 'lib';
END_VAR
    METHOD put : DINT
    VAR_INPUT
        s : STRING[20];
        a : ARRAY[0..3] OF DINT;
        w : WSTRING[20];
    END_VAR
        text := s;
        put := a[0] + a[3];
    END_METHOD

    METHOD get : STRING[20]
        get := text;
    END_METHOD

    METHOD split : DINT
    VAR_OUTPUT
        head : STRING[20];
    END_VAR
    VAR_IN_OUT
        tail : STRING[20];
    END_VAR
        head := text;
        tail := 'cut';
        split := 3;
    END_METHOD

    METHOD by_ref : DINT
    VAR_INPUT
        r : REFERENCE TO STRING[20];
        p : REF_TO DINT;
    END_VAR
        r := text;
        by_ref := p^ + 1;
    END_METHOD

    PROPERTY_GET label : STRING[20]
        label := text;
    END_PROPERTY

    METHOD rename : DINT
    VAR_INPUT
        n : STRING[20];
    END_VAR
        text := n;
        rename := 7;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK LibExercise
VAR_INPUT
    t : INamed;
END_VAR
VAR_OUTPUT
    put_result : DINT;
    got : STRING[20];
    split_result : DINT;
    head : STRING[20];
    tail : STRING[20];
    by_ref_result : DINT;
    target : STRING[20];
    rename_result : DINT;
    label : STRING[20];
END_VAR
VAR
    arr : ARRAY[0..3] OF DINT := [1, 0, 0, 2];
    value : DINT := 41;
END_VAR
    tail := 'whole';
    put_result := t.put('abc', arr, "xy");
    got := t.get();
    split_result := t.split(head => head, tail := tail);
    by_ref_result := t.by_ref(target, REF(value));
    rename_result := t.rename('renamed');
    label := t.label;
END_FUNCTION_BLOCK

TYPE Mode : (slow, fast); END_TYPE

INTERFACE IOther
    METHOD id : DINT END_METHOD
END_INTERFACE

INTERFACE IExtra
    METHOD pick : DINT
    VAR_INPUT
        c : Mode;
        r : INT(0..10);
    END_VAR
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        others : ARRAY[0..1] OF IOther;
        p : POINTER TO STRING[10];
    END_VAR
    END_METHOD

    METHOD defaults : DINT
    VAR_INPUT
        d : STRING[10] := 'dflt';
        m : Mode := fast;
    END_VAR
    END_METHOD

    METHOD triple : ARRAY[0..2] OF DINT END_METHOD

    PROPERTY_SET name : STRING[20] END_PROPERTY
    PROPERTY_GET name : STRING[20] END_PROPERTY
    PROPERTY_GET pair : ARRAY[0..1] OF DINT END_PROPERTY
END_INTERFACE

INTERFACE IDeclaredOnly
    METHOD pick : DINT
    VAR_INPUT
        c : (cyan, magenta);
    END_VAR
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        n : ARRAY[0..1] OF ARRAY[0..1] OF STRING[5];
        row : ARRAY[0..1] OF STRING[5];
    END_VAR
    END_METHOD

    METHOD slot : REFERENCE TO DINT END_METHOD

    METHOD texts : DINT
    VAR_IN_OUT
        arr : ARRAY[*] OF STRING[4];
    END_VAR
    END_METHOD

    METHOD nested_pointers : DINT
    VAR_INPUT
        q : POINTER TO ARRAY[0..1] OF STRING[5];
        r : REF_TO ARRAY[0..1] OF DINT;
    END_VAR
    END_METHOD
END_INTERFACE

INTERFACE IWide
    METHOD wide : DINT
    VAR_INPUT
        t : WSTRING[9] := "wide";
    END_VAR
    END_METHOD
END_INTERFACE

INTERFACE IMore EXTENDS IOther, IWide
    METHOD take_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IOther;
    END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Other IMPLEMENTS IOther
VAR_INPUT
    v : DINT;
END_VAR
    METHOD id : DINT
        id := v;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK LibExtra IMPLEMENTS IExtra
VAR
    text : STRING[20] := 'start';
    base : DINT;
END_VAR
    METHOD pick : DINT
    VAR_INPUT
        c : Mode;
        r : INT(0..10);
    END_VAR
        IF c = fast THEN
            pick := base + 100 + r;
        ELSE
            pick := base + r;
        END_IF
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        others : ARRAY[0..1] OF IOther;
        p : POINTER TO STRING[10];
    END_VAR
        text := 'grid';
        p^ := 'pw';
        grid := base + others[0].id() + others[1].id();
    END_METHOD

    METHOD defaults : DINT
    VAR_INPUT
        d : STRING[10] := 'dflt';
        m : Mode := fast;
    END_VAR
        text := d;
        defaults := base + m;
    END_METHOD

    METHOD triple : ARRAY[0..2] OF DINT
        triple[0] := base + 1;
        triple[1] := base + 2;
        triple[2] := base + 3;
    END_METHOD

    PROPERTY_SET name : STRING[20]
        text := name;
    END_PROPERTY

    PROPERTY_GET name : STRING[20]
        name := text;
    END_PROPERTY

    PROPERTY_GET pair : ARRAY[0..1] OF DINT
        pair[0] := base + 7;
        pair[1] := base + 8;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK LibExtraExercise
VAR_INPUT
    e : IExtra;
END_VAR
VAR_OUTPUT
    pick_result : DINT;
    grid_result : DINT;
    grid_text : STRING[20];
    written : STRING[10];
    defaults_result : DINT;
    defaults_text : STRING[20];
    triple : ARRAY[0..2] OF DINT;
    name : STRING[20];
    pair : ARRAY[0..1] OF DINT;
END_VAR
VAR
    o1 : Other := (v := 10);
    o2 : Other := (v := 20);
    others : ARRAY[0..1] OF IOther;
END_VAR
    others[0] := o1;
    others[1] := o2;
    pick_result := e.pick(fast, 4);
    grid_result := e.grid(others, REF(written));
    grid_text := e.name;
    defaults_result := e.defaults('given', slow);
    defaults_text := e.name;
    triple := e.triple();
    e.name := 'set';
    name := e.name;
    pair := e.pair;
END_FUNCTION_BLOCK

FUNCTION_BLOCK LibMore IMPLEMENTS IMore
    METHOD id : DINT
        id := 10;
    END_METHOD

    METHOD wide : DINT
    VAR_INPUT
        t : WSTRING[9] := "wide";
    END_VAR
        wide := 100;
    END_METHOD

    METHOD take_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IOther;
    END_VAR
        take_ref := 1000 + x.id();
    END_METHOD
END_FUNCTION_BLOCK

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
