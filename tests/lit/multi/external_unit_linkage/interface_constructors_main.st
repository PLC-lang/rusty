FUNCTION_BLOCK MainText IMPLEMENTS INamed
VAR
    text : STRING[20] := 'main';
END_VAR
    METHOD put : DINT
    VAR_INPUT
        s : STRING[20];
        a : ARRAY[0..3] OF DINT;
        w : WSTRING[20];
    END_VAR
        text := s;
        put := 100 + a[0] + a[3] + LEN(w);
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
        head := 'mhead';
        tail := 'mtail';
        split := 100 + LEN(head);
    END_METHOD

    METHOD by_ref : DINT
    VAR_INPUT
        r : REFERENCE TO STRING[20];
        p : REF_TO DINT;
    END_VAR
        r := 'mref';
        by_ref := p^ + 100;
    END_METHOD

    PROPERTY_GET label : STRING[20]
        label := CONCAT('M-', text);
    END_PROPERTY

    METHOD rename : DINT
    VAR_INPUT
        n : STRING[20];
    END_VAR
        text := n;
        rename := 100 + LEN(n);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK MainExtra IMPLEMENTS IExtra
VAR
    text : STRING[20] := 'start';
    base : DINT := 1000;
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
        text := 'mgrid';
        p^ := 'mpw';
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

FUNCTION_BLOCK MainMore IMPLEMENTS IMore
    METHOD id : DINT
        id := 20;
    END_METHOD

    METHOD wide : DINT
    VAR_INPUT
        t : WSTRING[9] := "wide";
    END_VAR
        wide := 200 + LEN(t);
    END_METHOD

    METHOD take_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IOther;
    END_VAR
        take_ref := 2000 + x.id();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION print_results : DINT
VAR_INPUT
    exercise : LibExercise;
END_VAR
    printf('%d$N', exercise.put_result);
    printf('%s$N', REF(exercise.got));
    printf('%d$N', exercise.split_result);
    printf('%s %s$N', REF(exercise.head), REF(exercise.tail));
    printf('%d$N', exercise.by_ref_result);
    printf('%s$N', REF(exercise.target));
    printf('%d$N', exercise.rename_result);
    printf('%s$N', REF(exercise.label));
END_FUNCTION

FUNCTION print_extra : DINT
VAR_INPUT
    ex : LibExtraExercise;
END_VAR
    printf('%d %d %s %s$N', ex.pick_result, ex.grid_result, REF(ex.grid_text), REF(ex.written));
    printf('%d %s$N', ex.defaults_result, REF(ex.defaults_text));
    printf('%d %d %d$N', ex.triple[0], ex.triple[1], ex.triple[2]);
    printf('%s %d %d$N', REF(ex.name), ex.pair[0], ex.pair[1]);
END_FUNCTION

FUNCTION main : DINT
VAR
    lib_text : LibText;
    main_text : MainText;
    exercise : LibExercise;
    lib_extra : LibExtra;
    main_extra : MainExtra;
    extra_exercise : LibExtraExercise;
    lib_more : LibMore;
    main_more : MainMore;
END_VAR
    exercise(t := lib_text);
    print_results(exercise);
    exercise(t := main_text);
    print_results(exercise);
    extra_exercise(e := lib_extra);
    print_extra(extra_exercise);
    extra_exercise(e := main_extra);
    print_extra(extra_exercise);
    printf('%d %d$N', more_of(lib_more), more_of(main_more));
END_FUNCTION
