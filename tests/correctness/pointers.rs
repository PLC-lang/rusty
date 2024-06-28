// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{compile_and_run, MainType};

mod references;

#[test]
fn pointer_test_builtin() {
    let function = r"
TYPE MyStruct: STRUCT  x: DINT; y: DINT; END_STRUCT END_TYPE
TYPE MyRef : REF_TO REF_TO DINT; END_TYPE

FUNCTION main : DINT
    main := foo();
END_FUNCTION

FUNCTION foo : DINT
VAR
                x : DINT;
                s : MyStruct;
                u,y : REF_TO DINT;
                z : REF_TO REF_TO DINT;
                v : MyRef;

END_VAR
u := REF(s.x);
y := u;
z := ADR(y);
s.x := 9;
z^^ := y^*2;
v := z;
y^ := v^^*2;

foo := y^ + 1;
END_FUNCTION
 ";

    let mut maintype = MainType::default();

    let res: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(37, res);
}
#[test]
fn pointer_test() {
    let function = r"
TYPE MyStruct: STRUCT  x: DINT; y: DINT; END_STRUCT END_TYPE
TYPE MyRef : REF_TO REF_TO DINT; END_TYPE

FUNCTION main : DINT
    main := foo();
END_FUNCTION

FUNCTION foo : DINT
VAR
                x : DINT;
                s : MyStruct;
                u,y : REF_TO DINT;
                z : REF_TO REF_TO DINT;
                v : MyRef;

END_VAR
u := REF(s.x);
y := u;
z := REF(y);
s.x := 9;
z^^ := y^*2;
v := z;
y^ := v^^*2;

foo := y^;
END_FUNCTION
 ";

    let mut maintype = MainType::default();

    let res: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(36, res);
}

#[test]
fn binary_expressions_for_pointers() {
    #[derive(Default)]
    #[repr(C)]
    struct Main {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        equal: bool,
        not_equal: bool,
        less: bool,
        greater: bool,
        less_or_equal: bool,
        greater_or_equal: bool,
    }

    let function = "
    PROGRAM main
    VAR
        a : CHAR;
        b : CHAR;
        c : CHAR;
        d : CHAR;
        e : CHAR;
        equal : BOOL;
        not_equal : BOOL;
        less : BOOL;
        greater : BOOL;
        less_or_equal : BOOL;
        greater_or_equal : BOOL;
    END_VAR
    VAR_TEMP
        arr : ARRAY[0..3] OF CHAR := ['a','b','c','d'];
        ptr : REF_TO CHAR;
        negative : INT := -1;
    END_VAR
        ptr := REF(arr);

        ptr := ptr + 1 + 1;
        a := ptr^;
        ptr := ptr + 1;
        b := ptr^;
        ptr := ptr - 1;
        c := ptr^;
        ptr := ptr + negative;
        d := ptr^;
        ptr := ptr - negative;
        e := ptr^;

        equal := ptr = ptr;
        not_equal := ptr <> ptr;
        less := ptr < ptr + 1;
        greater := ptr > ptr - 1;
        less_or_equal := ptr <= ptr;
        greater_or_equal := ptr >= ptr;
    END_PROGRAM
    ";
    let mut main = Main::default();
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!(main.a, "c".as_bytes()[0]);
    assert_eq!(main.b, "d".as_bytes()[0]);
    assert_eq!(main.c, "c".as_bytes()[0]);
    assert_eq!(main.d, "b".as_bytes()[0]);
    assert_eq!(main.e, "c".as_bytes()[0]);
    assert!(main.equal);
    assert!(!main.not_equal);
    assert!(main.less);
    assert!(main.greater);
    assert!(main.less_or_equal);
    assert!(main.greater_or_equal);
}

#[test]
fn binary_expressions_for_pointers_with_function_return() {
    #[derive(Default)]
    struct Main {
        a: u8,
        b: u8,
        c: u8,
    }

    let function = "
    FUNCTION len : INT
        len := 1;
    END_FUNCTION
    PROGRAM main
    VAR
        a : CHAR;
        b : CHAR;
        c : CHAR;
    END_VAR
    VAR_TEMP
        arr : ARRAY[0..2] OF CHAR := ['a','b', 'c'];
        ptr : REF_TO CHAR;
    END_VAR
        ptr := REF(arr);

        a := ptr^;
        ptr := REF(arr[0]) + len() + 1;
        b := ptr^;
        ptr := ptr - len() - 1;
        c := ptr^;
    END_PROGRAM
    ";
    let mut main = Main::default();
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!(main.a, "a".as_bytes()[0]);
    assert_eq!(main.b, "c".as_bytes()[0]);
    assert_eq!(main.c, "a".as_bytes()[0]);
}

#[test]
fn value_behind_function_block_pointer_is_assigned_to_correctly() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        a: bool,
        b: bool,
        file: FileT,
        file_open: Option<&'static FileT>,
    }

    #[repr(C)]
    #[derive(Default)]
    struct FileT {
        var1: bool,
        var2: bool,
        out1: bool,
        out2: bool,
    }

    let src = r#"
        FUNCTION_BLOCK file_t
        VAR_INPUT
            var1 : BOOL;
            var2 : BOOL;
        END_VAR
        VAR_OUTPUT
            out1 : BOOL;
            out2 : BOOL;
        END_VAR
            out1 := var1;
            out2 := var2;
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            a: BOOL;
            b: BOOL;
            file : file_t;
            FileOpen : REF_TO file_t;
        END_VAR
            FileOpen := REF(file);
            FileOpen^(var1 := FALSE, var2:=TRUE, out1 => a, out2 => b);
        END_PROGRAM
        "#;

    let mut maintype = MainType::default();
    let _: i32 = compile_and_run(src, &mut maintype);
    assert!(!maintype.a);
    assert!(maintype.b);
}

#[test]
fn reference_assignment() {
    let function = r"
        FUNCTION main : DINT
            VAR
                a : REF_TO DINT;
                b : DINT := 5;
            END_VAR

            a REF= b;
            main := a^;
        END_FUNCTION
    ";

    let res: i32 = compile_and_run(function.to_string(), &mut MainType::default());
    assert_eq!(5, res);
}

#[test]
fn reference_to_assignment() {
    let function = r"
        FUNCTION main : DINT
            VAR
                a : REFERENCE TO DINT;
                b : DINT := 5;
            END_VAR
            a REF= b;
            main := a;
        END_FUNCTION
    ";

    let res: i32 = compile_and_run(function, &mut MainType::default());
    assert_eq!(5, res);
}

#[test]
fn reference_to_variable_referencing_other_reference_to_variable() {
    let function = r"
        FUNCTION main : DINT
            VAR
                foo : REFERENCE TO DINT;
                bar : REFERENCE TO DINT;
                qux : DINT;
            END_VAR
            
            bar REF= qux;
            foo REF= bar;
            qux := 5;

            main := foo; // foo -> bar -> qux
        END_FUNCTION
    ";

    let res: i32 = compile_and_run(function, &mut MainType::default());
    assert_eq!(5, res);
}

#[test]
fn reference_to_variable_referencing_itself() {
    let function = r"
        FUNCTION main : DINT
            VAR
                foo : REFERENCE TO DINT;
                bar : REFERENCE TO DINT;
                qux : DINT;
            END_VAR
            
            foo REF= bar;
            bar REF= qux;

            bar REF= bar; 
            qux := 5;

            main := bar; // bar (-> bar) -> qux
        END_FUNCTION
    ";

    let res: i32 = compile_and_run(function, &mut MainType::default());
    assert_eq!(5, res);
}

#[test]
fn reference_to_variable_referencing_struct() {
    let function = r"
        TYPE Transaction : STRUCT
            id      : DINT;
            amount  : DINT;
            message : STRING;
        END_STRUCT END_TYPE

        FUNCTION main : DINT
            VAR
                txn     : Transaction := (id := 1, amount := 5, message := 'whats up');
                refTxn  : REFERENCE TO Transaction;
            END_VAR

            refTxn REF= txn;
            main := refTxn.amount;
        END_FUNCTION
    ";

    let res: i32 = compile_and_run(function, &mut MainType::default());
    assert_eq!(5, res);
}
