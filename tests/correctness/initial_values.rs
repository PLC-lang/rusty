// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
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
    MainType {
        x: 0,
        x_: 0,
        y: false,
        y_: false,
        z: 0.0,
        z_: 0.0,
    }
}
#[test]
fn initia_values_of_programs_members() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert_eq!(true, maintype.y);
    assert_eq!(false, maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initia_values_of_programs_members_using_constants() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert_eq!(true, maintype.y);
    assert_eq!(false, maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initia_values_of_functionblock_members() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert_eq!(true, maintype.y);
    assert_eq!(false, maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initia_values_of_function_members() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(88, maintype.y);
    assert_eq!(99, maintype.z);
}

#[test]
fn initia_values_of_struct_type_members() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(77, maintype.x);
    assert_eq!(0, maintype.x_);
    assert_eq!(true, maintype.y);
    assert_eq!(false, maintype.y_);
    assert_almost_eq!(9.1415, maintype.z, f32::EPSILON);
    assert_almost_eq!(0.0, maintype.z_, f32::EPSILON);
}

#[test]
fn initia_values_of_alias_type() {
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

    compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(7, maintype.x);
    assert_eq!(8, maintype.x_);
    assert_eq!(true, maintype.y);
    assert_eq!(false, maintype.y_);
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

    compile_and_run(function.to_string(), &mut maintype);
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
    assert_eq!(true, maintype.h0);
    assert_eq!(false, maintype.h2);
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

    let mut maintype: MultiDimArrayProgram = MultiDimArrayProgram {
        a0: 0,
        a1: 0,
        a2: 0,
        a3: 0,
    };

    compile_and_run(function.to_string(), &mut maintype);
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

    let mut maintype: ArrayOfArrayProgram = ArrayOfArrayProgram {
        a1: 0,
        a2: 0,
        a3: 0,
        a4: 0,
        a5: 0,
        a6: 0,
        a7: 0,
        a8: 0,
    };

    compile_and_run(function.to_string(), &mut maintype);
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

    let mut maintype = RealsAndFloats {
        f1: 0.0,
        f2: 0.0,
        r1: 0.0,
        r2: 0.0,
    };

    compile_and_run(function.to_string(), &mut maintype);
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

    let mut maintype = StructProgram {
        x: 0,
        y: 0,
        arr1: 0,
        arr3: 0,
        f: 0.0,
    };

    compile_and_run(src.to_string(), &mut maintype);
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

    let mut maintype = StructProgram {
        x: 0,
        y: 0,
        arr1: 0,
        arr3: 0,
        f: 0.0,
    };

    compile_and_run(src.to_string(), &mut maintype);
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

    compile_and_run(src.to_string(), &mut maintype);
    assert_eq!(
        &maintype.mystring1[0..8],
        [97, 98, 99, 100, 101, 102, 103, 0]
    ); // abcdefg
    assert_eq!(&maintype.mystring1[9..26], [0; 17]); //rest is blank

    assert_eq!(&maintype.mystring2[0..8], [65, 66, 67, 68, 69, 70, 71, 0]); // ABCDEFG
    assert_eq!(&maintype.mystring2[9..26], [0; 17]); //rest is blank

    assert_eq!(maintype.string1, [0; 81]); // blank string

    assert_eq!(maintype.string2[0..6], [113, 119, 101, 114, 116, 0]); // qwert
    assert_eq!(maintype.string2[7..81], [0; 74]); // rest is blank

    assert_eq!(
        maintype.string3[0..6],
        [113 - 32, 119 - 32, 101 - 32, 114 - 32, 116 - 32, 0]
    ); // QWERT
    assert_eq!(maintype.string3[7..21], [0; 14]); // rest is blank
}
