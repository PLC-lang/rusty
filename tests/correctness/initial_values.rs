// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    x: i32,
    x_: i32,
    y: bool,
    y_: bool,
    z: f32,
    z_: f32,
}

struct ThreeInts {
    x: i32,
    y: i32,
    z: i32,
}

fn new() -> MainType {
    MainType { x: 0, x_: 0, y: false, y_: false, z: 0.0, z_: 0.0 }
}
#[test]
fn initial_values_of_programs_members() {
    let function = r"
        PROGRAM other
        VAR
            x   : DINT := 77;
            x_  : DINT;
            y   : BOOL := TRUE;
            y_  : BOOL;
            z   : REAL := 9.1415;
            z_  : REAL;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.x;
            x_ := other.x_;
            y := other.y;
            y_ := other.y_;
            z := other.z;
            z_ := other.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert!(maintype.y);
    assert!(!maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initial_values_of_programs_members_using_constants() {
    let function = r"
        VAR_GLOBAL CONSTANT
            cX      : DINT := 70;
            cSeven  : DINT := 7;
            cT      : BOOL := TRUE;
            cF      : BOOL := FALSE;
            cR      : REAL := 9.1;
            cFr     : REAL := 0.0415;
        END_VAR

        PROGRAM other
        VAR
            x   : DINT := cX + cSeven;
            x_  : DINT;
            y   : BOOL := cT XOR cF;
            y_  : BOOL;
            z   : REAL := cR + cFr;
            z_  : REAL;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.x;
            x_ := other.x_;
            y := other.y;
            y_ := other.y_;
            z := other.z;
            z_ := other.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert!(maintype.y);
    assert!(!maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initial_values_of_functionblock_members() {
    let function = r"
        FUNCTION_BLOCK MyFB
        VAR
            x   : DINT := 77;
            x_  : DINT;
            y   : BOOL := TRUE;
            y_  : BOOL;
            z   : REAL := 9.1415;
            z_  : REAL;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM other
            VAR myFB: MyFB; END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.myFB.x;
            x_ := other.myFB.x_;
            y := other.myFB.y;
            y_ := other.myFB.y_;
            z := other.myFB.z;
            z_ := other.myFB.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert!(maintype.y);
    assert!(!maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initial_values_of_function_members() {
    let function = r"
        FUNCTION other : DINT
        VAR
        x   : DINT := 77;
        y   : DINT := 88;
        z   : DINT := 99;
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := x;
            ELSIF index = 1 THEN
                other := y;
            ELSE
                other := z;
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            x : DINT;
            y : DINT;
            z : DINT;
        END_VAR
            x := other(index := 0);
            y := other(index := 1);
            z := other(index := 2);
        END_PROGRAM
        ";

    let mut maintype = ThreeInts { x: 0, y: 0, z: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(88, maintype.y);
    assert_eq!(99, maintype.z);
}

#[test]
fn initial_values_of_struct_type_members() {
    let function = r"
        TYPE MyStruct :
            STRUCT
                x   : DINT := 77;
                x_  : DINT;
                y   : BOOL := TRUE;
                y_  : BOOL;
                z   : REAL := 9.1415;
                z_  : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL
            other: MyStruct;
        END_VAR

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.x;
            x_ := other.x_;
            y := other.y;
            y_ := other.y_;
            z := other.z;
            z_ := other.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert!(maintype.y);
    assert!(!maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initial_values_of_alias_type() {
    let function = r"
        TYPE MyInt  : DINT := 7; END_TYPE
        TYPE MyBool : BOOL := TRUE; END_TYPE
        TYPE MyReal : REAL := 5.67; END_TYPE

        VAR_GLOBAL
            gx   : MyInt;
            gxx  : MyInt := 8;
            gy   : MyBool;
            gyy   : MyBool := FALSE;
            gz   : MyReal;
            gzz  : MyReal := 1.23;
        END_VAR

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := gx;
            x_ := gxx;
            y := gy;
            y_ := gyy;
            z := gz;
            z_ := gzz;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(7, maintype.x);
    assert_eq!(8, maintype.x_);
    assert!(maintype.y);
    assert!(!maintype.y_);
    assert_almost_eq!(5.67, maintype.z, f32::EPSILON);
    assert_almost_eq!(1.23, maintype.z_, f32::EPSILON);
}

#[derive(Debug)]
#[repr(C)]
struct ArrayProgram {
    a0: i8,
    a2: i8,
    b0: i16,
    b2: i16,
    c0: i32,
    c2: i32,
    d0: i64,
    d2: i64,
    e0: u8,
    e2: u8,
    f0: u16,
    f2: u16,
    g0: u64,
    g2: u64,
    h0: bool,
    h2: bool,
}

#[test]
fn initial_values_in_single_dimension_array_variable() {
    let function = r"
        VAR_GLOBAL
          a : ARRAY[0..2] OF SINT  := [1, 2, 3];
          b : ARRAY[0..2] OF INT  := [4, 5, 6];
          c : ARRAY[0..2] OF DINT  := [7, 8, 9];
          d : ARRAY[0..2] OF LINT  := [10, 11, 12];
          _e : ARRAY[0..2] OF USINT  := [13, 14, 15];
          f : ARRAY[0..2] OF UINT  := [16, 17, 18];
          g : ARRAY[0..2] OF ULINT := [19, 20, 21];
          h : ARRAY[0..2] OF BOOL := [TRUE, FALSE, FALSE];
        END_VAR

        PROGRAM main
            VAR
                a0 : SINT;
                a2 : SINT;
                b0 : INT;
                b2 : INT;
                c0 : DINT;
                c2 : DINT;
                d0 : LINT;
                d2 : LINT;
                _e0 : USINT;
                _e2 : USINT;
                f0 : UINT;
                f2 : UINT;
                g0 : ULINT;
                g2 : ULINT;
                h0 : BOOL;
                h2 : BOOL;
            END_VAR
            a0 := a[0]; a2 := a[2];
            b0 := b[0]; b2 := b[2];
            c0 := c[0]; c2 := c[2];
            d0 := d[0]; d2 := d[2];
            _e0 := _e[0]; _e2 := _e[2];
            f0 := f[0]; f2 := f[2];
            g0 := g[0]; g2 := g[2];
            h0 := h[0]; h2 := h[2];
        END_PROGRAM";

    let mut maintype: ArrayProgram = ArrayProgram {
        a0: 0,
        a2: 0,
        b0: 0,
        b2: 0,
        c0: 0,
        c2: 0,
        d0: 0,
        d2: 0,
        e0: 0,
        e2: 0,
        f0: 0,
        f2: 0,
        g0: 0,
        g2: 0,
        h0: false,
        h2: true,
    };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(1, maintype.a0);
    assert_eq!(3, maintype.a2);
    assert_eq!(4, maintype.b0);
    assert_eq!(6, maintype.b2);
    assert_eq!(7, maintype.c0);
    assert_eq!(9, maintype.c2);
    assert_eq!(10, maintype.d0);
    assert_eq!(12, maintype.d2);
    assert_eq!(13, maintype.e0);
    assert_eq!(15, maintype.e2);
    assert_eq!(16, maintype.f0);
    assert_eq!(18, maintype.f2);
    assert_eq!(19, maintype.g0);
    assert_eq!(21, maintype.g2);
    assert!(maintype.h0);
    assert!(!maintype.h2);
}

#[derive(Debug)]
#[repr(C)]
struct MultiDimArrayProgram {
    a0: i8,
    a1: i8,
    a2: i8,
    a3: i8,
}

#[test]
fn initial_values_in_multi_dimension_array_variable() {
    let function = r"
        VAR_GLOBAL
          a : ARRAY[0..1, 3..4] OF SINT  := [1, 2, 3, 4];
        END_VAR

        PROGRAM main
            VAR
                a0 : SINT;
                a1 : SINT;
                a2 : SINT;
                a3 : SINT;
            END_VAR

            a0 := a[0,3];
            a1 := a[0,4];
            a2 := a[1,3];
            a3 := a[1,4];
        END_PROGRAM";

    let mut maintype: MultiDimArrayProgram = MultiDimArrayProgram { a0: 0, a1: 0, a2: 0, a3: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(1, maintype.a0);
    assert_eq!(2, maintype.a1);
    assert_eq!(3, maintype.a2);
    assert_eq!(4, maintype.a3);
}

#[derive(Debug)]
#[repr(C)]
struct ArrayOfArrayProgram {
    a1: i8,
    a2: i8,
    a3: i8,
    a4: i8,
    a5: i8,
    a6: i8,
    a7: i8,
    a8: i8,
}

#[test]
fn initial_values_in_array_of_array_variable() {
    let function = r"
        VAR_GLOBAL
          a : ARRAY[0..1] OF ARRAY[0..1] OF ARRAY[0..1] OF SINT  := [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
        END_VAR

        PROGRAM main
            VAR
                a1 : SINT;
                a2 : SINT;
                a3 : SINT;
                a4 : SINT;
                a5 : SINT;
                a6 : SINT;
                a7 : SINT;
                a8 : SINT;
            END_VAR

            a1 := a[0][0][0];
            a2 := a[0][0][1];
            a3 := a[0][1][0];
            a4 := a[0][1][1];
            a5 := a[1][0][0];
            a6 := a[1][0][1];
            a7 := a[1][1][0];
            a8 := a[1][1][1];
        END_PROGRAM";

    let mut maintype: ArrayOfArrayProgram =
        ArrayOfArrayProgram { a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0, a8: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(1, maintype.a1);
    assert_eq!(2, maintype.a2);
    assert_eq!(3, maintype.a3);
    assert_eq!(4, maintype.a4);
    assert_eq!(5, maintype.a5);
    assert_eq!(6, maintype.a6);
    assert_eq!(7, maintype.a7);
    assert_eq!(8, maintype.a8);
}

#[derive(Debug)]
#[repr(C)]
struct RealsAndFloats {
    f1: f32,
    f2: f32,
    r1: f64,
    r2: f64,
}

#[test]
fn real_initial_values_in_array_variable() {
    let function = r"
        VAR_GLOBAL
            f : ARRAY[0..1] OF REAL := [9.1415, 0.001];
            r : ARRAY[0..1] OF LREAL := [9.141592653589, 0.000000001];
        END_VAR

        PROGRAM main
            VAR
                f1 : REAL;
                f2 : REAL;
                r1 : LREAL;
                r2 : LREAL;
            END_VAR

            f1 := f[0];
            f2 := f[1];
            r1 := r[0];
            r2 := r[1];
        END_PROGRAM";

    let mut maintype = RealsAndFloats { f1: 0.0, f2: 0.0, r1: 0.0, r2: 0.0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_almost_eq!(9.1415, maintype.f1, f32::EPSILON);
    assert_almost_eq!(0.001, maintype.f2, f32::EPSILON);
    assert_almost_eq!(9.141592653589, maintype.r1, f64::EPSILON);
    assert_almost_eq!(0.000000001, maintype.r2, f64::EPSILON);
}

#[derive(Debug)]
#[repr(C)]
struct StructProgram {
    x: i32,
    y: i32,
    arr1: i16,
    arr3: i16,
    f: f32,
}

#[test]
fn initialization_of_complex_struct_instance() {
    let src = "
        TYPE MyPoint: STRUCT
          x: DINT;
          y: DINT;
        END_STRUCT
        END_TYPE

        TYPE MyStruct: STRUCT
          point: MyPoint;
          my_array: ARRAY[0..3] OF INT;
          f : REAL;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          a : MyStruct  := (
              point := (x := 1, y:= 2),
              my_array := [0,1,2,3],
              f := 7.89
            );
        END_VAR

        PROGRAM main
            VAR
                x : DINT;
                y : DINT;
                arr1 : INT;
                arr3 : INT;
                f : REAL;
            END_VAR

            x := a.point.x;
            y := a.point.y;
            arr1 := a.my_array[1];
            arr3 := a.my_array[3];
            f := a.f;
        END_PROGRAM
        ";

    let mut maintype = StructProgram { x: 0, y: 0, arr1: 0, arr3: 0, f: 0.0 };

    let _: i32 = compile_and_run(src.to_string(), &mut maintype);
    assert_eq!(1, maintype.x);
    assert_eq!(2, maintype.y);
    assert_eq!(1, maintype.arr1);
    assert_eq!(3, maintype.arr3);
    assert_almost_eq!(7.89, maintype.f, f32::EPSILON);
}

#[test]
fn initialization_of_complex_struct_instance_using_defaults() {
    // a.point.y and a.f are note initialized!
    let src = "
        TYPE MyReal : REAL := 9.1415; END_TYPE

        TYPE MyPoint: STRUCT
          x: DINT;
          y: DINT := 7;
        END_STRUCT
        END_TYPE

        TYPE MyStruct: STRUCT
          point: MyPoint;
          my_array: ARRAY[0..3] OF INT;
          f : MyReal;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          a : MyStruct  := (
              point := (x := 1),
              my_array := [0,1,2,3]
            );
        END_VAR

        PROGRAM main
            VAR
                x : DINT;
                y : DINT;
                arr1 : INT;
                arr3 : INT;
                f : REAL;
            END_VAR

            x := a.point.x;
            y := a.point.y;
            arr1 := a.my_array[1];
            arr3 := a.my_array[3];
            f := a.f;
        END_PROGRAM
        ";

    let mut maintype = StructProgram { x: 0, y: 0, arr1: 0, arr3: 0, f: 0.0 };

    let _: i32 = compile_and_run(src.to_string(), &mut maintype);
    assert_eq!(1, maintype.x);
    assert_eq!(7, maintype.y);
    assert_eq!(1, maintype.arr1);
    assert_eq!(3, maintype.arr3);
    assert_almost_eq!(9.1415, maintype.f, f32::EPSILON);
}

#[derive(Debug)]
#[repr(C)]
struct StringStruct {
    mystring1: [i8; 26],
    mystring2: [i8; 26],
    string1: [i8; 81],
    string2: [i8; 81],
    string3: [i8; 21],
}

#[test]
fn initialization_of_string_variables() {
    // a.point.y and a.f are note initialized!
    let src = "
        TYPE MyString : STRING[25] := 'abcdefg'; END_TYPE

        TYPE StringStruct : STRUCT
                mystring: MyString;
                mystring2: MyString := 'ABCDEFG';
                string1 : STRING;
                string2 : STRING := 'qwert';
                string3 : STRING[20] := 'QWERT';
        END_STRUCT
        END_TYPE

        VAR_GLOBAL g : StringStruct; END_VAR

        PROGRAM main
            VAR
                mystring: MyString := 'xxx';
                mystring2: MyString;
                string1 : STRING := 'xxx';
                string2 : STRING := 'xxx';
                string3 : STRING[20];
            END_VAR

            mystring := g.mystring;
            mystring2 := g.mystring2;
            string1 := g.string1;
            string2 := g.string2;
            string3 := g.string3;
        END_PROGRAM
        ";

    let mut maintype = StringStruct {
        mystring1: [1; 26],
        mystring2: [1; 26],
        string1: [1; 81],
        string2: [1; 81],
        string3: [1; 21],
    };

    let _: i32 = compile_and_run(src.to_string(), &mut maintype);
    assert_eq!(&maintype.mystring1[0..8], [97, 98, 99, 100, 101, 102, 103, 0]); // abcdefg
                                                                                // assert_eq!(&maintype.mystring1[9..26], [0; 17]); //rest is blank

    assert_eq!(&maintype.mystring2[0..8], [65, 66, 67, 68, 69, 70, 71, 0]); // ABCDEFG
                                                                            // assert_eq!(&maintype.mystring2[9..26], [0; 17]); //rest is blank

    assert_eq!(maintype.string1[0], 0); // blank string

    assert_eq!(maintype.string2[0..6], [113, 119, 101, 114, 116, 0]); // qwert
                                                                      // assert_eq!(maintype.string2[7..81], [0; 74]); // rest is blank

    assert_eq!(maintype.string3[0..6], [113 - 32, 119 - 32, 101 - 32, 114 - 32, 116 - 32, 0]);
    // QWERT
    // assert_eq!(maintype.string3[7..21], [0; 14]); // rest is blank
}

#[derive(Debug, PartialEq)]
#[repr(C)]
struct FourInts {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

#[test]
fn initialization_of_function_variables() {
    let function = r"
        FUNCTION other : DINT
        VAR
            a   : DINT;
            b   : DINT := 10;
            c   : ARRAY[0..2] OF DINT := [10,20];
            d   : ARRAY[0..2] OF DINT;
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := a;
            ELSIF index = 1 THEN
                other := b;
            ELSIF index = 2 THEN
                other := c[1];
            ELSE
                other := d[0];
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := other(index := 0);
            b := other(index := 1);
            c := other(index := 2);
            d := other(index := 3);
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(0, maintype.a);
    assert_eq!(10, maintype.b);
    assert_eq!(20, maintype.c);
    assert_eq!(0, maintype.d);
}

#[test]
fn initialization_of_struct_in_fb() {
    let function = r"
        TYPE str : STRUCT
            a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT;
        END_STRUCT END_TYPE
        VAR_GLOBAL
            fb : other;
        END_VAR
        FUNCTION_BLOCK other
        VAR
          a : str;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := fb.a.a;
            b := fb.a.b;
            c := fb.a.c;
            d := fb.a.d;
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(10, maintype.a);
    assert_eq!(20, maintype.b);
    assert_eq!(30, maintype.c);
    assert_eq!(0, maintype.d);
}
#[test]
fn initialization_of_struct_in_prg() {
    let function = r"
        TYPE str : STRUCT
            a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT;
        END_STRUCT END_TYPE
        PROGRAM other
        VAR
          a : str;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := other.a.a;
            b := other.a.b;
            c := other.a.c;
          d := other.a.d;
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(10, maintype.a);
    assert_eq!(20, maintype.b);
    assert_eq!(30, maintype.c);
    assert_eq!(0, maintype.d);
}

#[test]
fn initialization_of_struct_ref_in_fb_in_function() {
    let function = r"
        TYPE str : STRUCT
            a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT;
        END_STRUCT END_TYPE
        FUNCTION_BLOCK fb
        VAR
          a : str;
        END_VAR
        END_FUNCTION_BLOCK
        FUNCTION other : DINT
        VAR
          x : fb;
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := x.a.a;
            ELSIF index = 1 THEN
                other := x.a.b;
            ELSIF index = 2 THEN
                other := x.a.c;
            ELSE
                other := x.a.d;
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := other(index := 0);
            b := other(index := 1);
            c := other(index := 2);
            d := other(index := 3);
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(10, maintype.a);
    assert_eq!(20, maintype.b);
    assert_eq!(30, maintype.c);
    assert_eq!(0, maintype.d);
}
#[test]
fn initialization_of_struct_ref_in_function() {
    let function = r"
        TYPE str : STRUCT
            a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT;
        END_STRUCT END_TYPE
        FUNCTION other : DINT
        VAR
          a : str;
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := a.a;
            ELSIF index = 1 THEN
                other := a.b;
            ELSIF index = 2 THEN
                other := a.c;
            ELSE
                other := a.d;
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := other(index := 0);
            b := other(index := 1);
            c := other(index := 2);
            d := other(index := 3);
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(10, maintype.a);
    assert_eq!(20, maintype.b);
    assert_eq!(30, maintype.c);
    assert_eq!(0, maintype.d);
}

#[test]
fn initialization_of_struct_in_function() {
    let function = r"
        FUNCTION other : DINT
        VAR
            a : STRUCT a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT; END_STRUCT
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := a.a;
            ELSIF index = 1 THEN
                other := a.b;
            ELSIF index = 2 THEN
                other := a.c;
            ELSE
                other := a.d;
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            d : DINT;
        END_VAR
            a := other(index := 0);
            b := other(index := 1);
            c := other(index := 2);
            d := other(index := 3);
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(10, maintype.a);
    assert_eq!(20, maintype.b);
    assert_eq!(30, maintype.c);
    assert_eq!(0, maintype.d);
}

#[test]
fn initialized_array_in_function() {
    let function = "
        FUNCTION foo : ARRAY[-1..2] OF DINT
        VAR
            arr_var : ARRAY[-1..2] OF DINT := [77,20,300,4000];
        END_VAR
            foo := arr_var;
        END_FUNCTION

        PROGRAM main
            VAR_INPUT
                a,b,c,d : DINT;
            END_VAR
            VAR_TEMP
                arr_var : ARRAY[-1..2] OF DINT;
            END_VAR

            arr_var := foo();
            a := arr_var[-1];
            b := arr_var[0];
            c := arr_var[1];
            d := arr_var[2];
        END_PROGRAM
        ";

    let mut maintype = FourInts { a: 0, b: 0, c: 0, d: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(FourInts { a: 77, b: 20, c: 300, d: 4000 }, maintype);
}

#[test]
fn array_test() {
    let function = "
        VAR_GLOBAL
            u,v,w,x : ULINT;
        END_VAR

        FUNCTION foo : ARRAY[-1..2] OF DINT
        VAR_INPUT
            arr_var : ARRAY[-1..2] OF DINT;
        END_VAR
            //main := arr_var;
            //main[-1] := 1;

            u := REF(arr_var[0]);
            v := REF(arr_var[1]);
            w := REF(arr_var[2]);
            x := REF(arr_var[3]);

            main.a := 99;
        END_FUNCTION

        PROGRAM main
            VAR_INPUT
                a,b,c,d : ULINT;
            END_VAR
            VAR_TEMP
                arr_var : ARRAY[-1..2] OF DINT := [77,20,300,4000];
            END_VAR
            a := 1; b:=2; c:=3; d:=4;

            foo(arr_var);

            a := u;
            b := v;
            c := w;
            d := x;
        END_PROGRAM
        ";

    #[derive(Debug)]
    #[repr(C)]
    struct T {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let mut maintype = T { a: 0, b: 0, c: 0, d: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    println!("{}, {}, {}, {}", maintype.a, maintype.b, maintype.c, maintype.d);
    println!("{}, {}, {}", maintype.b - maintype.a, maintype.c - maintype.b, maintype.d - maintype.c);
}

#[test]
fn initialized_array_type_in_function() {
    let function = "
    TYPE arr : ARRAY[-1..2] OF DINT := [1,2,3,4]; END_TYPE
        FUNCTION main : arr
        VAR
            arr_var : arr;
        END_VAR
            main := arr_var;
        END_FUNCTION
        ";
    #[allow(dead_code)]
    struct MainType {
        arr: [i32; 4],
    }
    let mut maintype = MainType { arr: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([1, 2, 3, 4], maintype.arr);
}

#[test]
fn initialized_array_in_program() {
    let function = "
        PROGRAM target
        VAR
            arr_var : ARRAY[-1..2] OF DINT := [1,2,3,4];
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            arr_var : ARRAY[-1..2] OF DINT;
        END_VAR
            arr_var := target.arr_var;
        END_PROGRAM
        ";

    #[allow(dead_code)]
    struct MainType {
        arr: [i32; 4],
    }
    let mut maintype = MainType { arr: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([1, 2, 3, 4], maintype.arr);
}

#[test]
fn initialized_array_type_in_program() {
    let function = "
        TYPE arr : ARRAY[-1..2] OF DINT := [1,2,3,4]; END_TYPE

        PROGRAM target
        VAR
            arr_var : arr;
        END_VAR
        END_PROGRAM

        PROGRAM main
            VAR
                arr_var : ARRAY[-1..2] OF DINT;
            END_VAR

           arr_var := target.arr_var;
        END_PROGRAM
        ";
    #[allow(dead_code)]
    struct MainType {
        arr: [i32; 4],
    }
    let mut maintype = MainType { arr: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([1, 2, 3, 4], maintype.arr);
}

#[test]
fn intial_values_diverge_from_type() {
    let function = "
    TYPE arr : ARRAY[-1..2] OF DINT := [1,2,3,4]; END_TYPE
    TYPE myInt : DINT := 4; END_TYPE

    PROGRAM target
    VAR
        arr_var : arr := [5,6,7,8];
        i : myInt := 5;
    END_VAR
    END_PROGRAM

    PROGRAM main
    VAR
        arr_var : arr;
        i : myInt;
    END_VAR
    arr_var := target.arr_var;
    i := target.i;
    END_PROGRAM
    ";
    #[allow(dead_code)]
    struct MainType {
        arr: [i32; 4],
        my_int: i32,
    }
    let mut maintype = MainType { arr: [0; 4], my_int: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([5, 6, 7, 8], maintype.arr);
    assert_eq!(5, maintype.my_int);
}

#[test]
fn initial_value_of_function_return_dint() {
    // GIVEN a DataType myInt with a default = 4
    // AND a function that returns a myINT

    // WHEN I only increment the function's return before returning it
    let function = "

    TYPE myInt : DINT := 4; END_TYPE

    FUNCTION target : myInt
        target := target + 1;
    END_FUNCTION

    PROGRAM main
    VAR
        i : DINT;
    END_VAR

    i := target();
    END_PROGRAM
    ";
    #[allow(dead_code)]
    struct MainType {
        i: i32,
    }

    // THEN i expect to get 5 (hence the return type's default is 4)
    let mut maintype = MainType { i: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(5, maintype.i);
}

#[test]
fn initial_value_of_function_return_array() {
    // GIVEN an Array-DataType myArray with a default = [1,2,3,4]
    // AND a function that returns a myArray

    // WHEN I only increment the function's return at position [2] before returning it
    let function = "
    TYPE myArray : ARRAY[0..3] OF DINT := [1,2,3,4]; END_TYPE

    FUNCTION target : myArray
        target[2] := target[2] + 1;
    END_FUNCTION

    PROGRAM main
    VAR
        arr : ARRAY[0..3] OF DINT;
    END_VAR
        arr := target();
    END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug)]
    struct MainType {
        arr: [i32; 4],
    }

    // THEN i expect to get [1,2,4,4]
    let mut maintype = MainType { arr: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!([1, 2, 4, 4], maintype.arr);
}

#[test]
fn initial_value_of_function_return_struct() {
    // GIVEN an Struct-DataType myStruct with a default = { a = 10, b = 20, c = 30 }
    // AND a function that returns a myArray

    // WHEN I only increment a by 1, b by 2 and c by 3 before returning it
    let function = "

    TYPE myStruct : STRUCT
            a : DINT := 10;
            b : DINT := 20;
            c : DINT := 30;
        END_STRUCT
    END_TYPE

    FUNCTION target : myStruct
        target.a := target.a + 1;
        target.b := target.b + 2;
        target.c := target.c + 3;
    END_FUNCTION

    PROGRAM main
        VAR
            a,b,c : DINT;
            str : myStruct;
        END_VAR

        str := target();
        a := str.a;
        b := str.b;
        c := str.c;
    END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        buffer: [i32; 3],
    }
    let mut maintype = MainType { a: 0, b: 0, c: 0, buffer: [0; 3] };

    // THEN i expect to get { a = 11, b = 22, c = 33 }
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([11, 22, 33], [maintype.a, maintype.b, maintype.c]);
}

#[test]
fn initial_value_of_enums() {
    let src = "
    VAR_GLOBAL
        x_gl : (red, yellow, green) := 2;
    END_VAR

    PROGRAM main
    VAR
        a, b, c, d : DINT;
    END_VAR
    VAR_TEMP
        x : (redy := 1, yellowy := 2, greeny := 3) := redy;
        y : (x1 := redy, x2 := yellowy, x3 := greeny) := x1;
        z : (x5, x6, x7) := yellowy;
    END_VAR
        a := x;
        b := y;
        c := z;
        d := x_gl;
    END_PROGRAM";

    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    let mut maintype = MainType::default();

    let _: i32 = compile_and_run(src.to_string(), &mut maintype);
    assert_eq!(1, maintype.a);
    assert_eq!(1, maintype.b);
    assert_eq!(2, maintype.c);
    assert_eq!(2, maintype.d);
}

#[test]
fn initial_value_in_array_of_struct() {
    let function = "
    TYPE myStruct : STRUCT
            a, b : DINT;
            c : ARRAY[0..1] OF DINT;
        END_STRUCT
    END_TYPE

    VAR_GLOBAL CONSTANT
        str : myStruct := (a := 50, b := 60, c := [70, 80]);
    END_VAR

    PROGRAM main
    VAR_TEMP
        arr : ARRAY[0..1] OF myStruct := [(a := 10, b := 20, c := [30, 40]), str];
    END_VAR
    VAR
        a, b, c, d : DINT;
        e, f, g, h : DINT;
    END_VAR
        a := arr[0].a;
        b := arr[0].b;
        c := arr[0].c[0];
        d := arr[0].c[1];

        e := arr[1].a;
        f := arr[1].b;
        g := arr[1].c[0];
        h := arr[1].c[1];
    END_PROGRAM
    ";

    #[allow(dead_code)]
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
    }
    let mut maintype = MainType::default();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(
        [10, 20, 30, 40, 50, 60, 70, 80],
        [maintype.a, maintype.b, maintype.c, maintype.d, maintype.e, maintype.f, maintype.g, maintype.h]
    );
}

#[test]
fn array_of_struct_as_member_of_another_struct_and_variable_declaration_is_initialized() {
    let function = "
        TYPE STRUCT1 : STRUCT
                myInt : DINT;
                myArr : ARRAY[0..4] OF STRUCT2;
        END_STRUCT END_TYPE

        TYPE STRUCT2 : STRUCT
                x1 : DINT;
                x2 : DINT;
        END_STRUCT END_TYPE

        PROGRAM target
            VAR
                str : ARRAY[0..4] OF STRUCT1 := [
                    (myInt := 1, myArr := [(x1 := 0, x2 := 128), (x1 := 1, x2 := 1024)]),
                    (myInt := 2, myArr := [(x1 := 0, x2 := 256), (x1 := 1, x2 := 2048)])
                ];
            END_VAR
        END_PROGRAM

        PROGRAM main
            VAR
                a0, b0, c0, d0, e0 : DINT;
                a1, b1, c1, d1, e1 : DINT;
            END_VAR

            a0 := target.str[0].myInt;
            b0 := target.str[0].myArr[0].x1;
            c0 := target.str[0].myArr[0].x2;
            d0 := target.str[0].myArr[1].x1;
            e0 := target.str[0].myArr[1].x2;

            a1 := target.str[1].myInt;
            b1 := target.str[1].myArr[0].x1;
            c1 := target.str[1].myArr[0].x2;
            d1 := target.str[1].myArr[1].x1;
            e1 := target.str[1].myArr[1].x2;
        END_PROGRAM
       ";

    #[derive(Default)]
    struct MainType {
        a0: i32,
        b0: i32,
        c0: i32,
        d0: i32,
        e0: i32,

        a1: i32,
        b1: i32,
        c1: i32,
        d1: i32,
        e1: i32,
    }

    let mut maintype = MainType::default();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!([1, 0, 128, 1, 1024], [maintype.a0, maintype.b0, maintype.c0, maintype.d0, maintype.e0]);
    assert_eq!([2, 0, 256, 1, 2048], [maintype.a1, maintype.b1, maintype.c1, maintype.d1, maintype.e1]);
}
