// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;

#[test]
fn using_structs() {
    #[repr(C)]
    #[derive(Debug, Default)]
    struct MyStruct {
        field1: i8,
        field2: i16,
        field3: i32,
    }

    struct MainType {
        my_s: MyStruct,
    }

    let mut main_data = MainType { my_s: MyStruct { field1: 0, field2: 0, field3: 0 } };

    let testcode = r#"
    TYPE MyStruct:
        STRUCT
            Field1 : BYTE;
            Field2 : INT;
            Field3 : DINT;
        END_STRUCT
    END_TYPE

    PROGRAM main
    VAR
        myS: MyStruct;
    END_VAR

        myS.Field1 := 3;
        myS.Field2 := 7;
        myS.Field3 := myS.Field1 + myS.Field2;
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut main_data);
    assert_eq!(3, main_data.my_s.field1);
    assert_eq!(7, main_data.my_s.field2);
    assert_eq!(10, main_data.my_s.field3);
}

#[test]
fn using_nested_structs() {
    #[repr(C)]
    #[derive(Debug, Default)]
    struct MyInnerStruct {
        field1: i8,
        field2: i16,
        field3: i32,
    }

    #[repr(C)]
    #[derive(Debug, Default)]
    struct MyStruct {
        mys1: MyInnerStruct,
        mys2: MyInnerStruct,
        mys3: MyInnerStruct,
    }

    #[derive(Debug, Default)]
    struct MainType {
        my_s: MyStruct,
    }

    let mut main_data = MainType {
        my_s: MyStruct {
            mys1: MyInnerStruct { field1: 0, field2: 0, field3: 0 },
            mys2: MyInnerStruct { field1: 0, field2: 0, field3: 0 },
            mys3: MyInnerStruct { field1: 0, field2: 0, field3: 0 },
        },
    };

    let testcode = r#"
    TYPE MyInnerStruct: 
        STRUCT 
            innerField1 : BYTE;
            innerField2 : INT;
            innerField3 : DINT;
        END_STRUCT
    END_TYPE

    TYPE MyStruct:
        STRUCT
            str1 : MyInnerStruct;
            str2 : MyInnerStruct;
            str3 : MyInnerStruct;
        END_STRUCT
    END_TYPE

    PROGRAM main
    VAR
        myS: MyStruct;
    END_VAR

        myS.str1.innerField1 := 11;
        myS.str1.innerField2 := 12;
        myS.str1.innerField3 := 13;
        
        myS.str2.innerField1 := 21;
        myS.str2.innerField2 := 22;
        myS.str2.innerField3 := 23;

        myS.str3.innerField1 := myS.str1.innerField1 + myS.str2.innerField1;
        myS.str3.innerField2 := myS.str1.innerField2 + myS.str2.innerField2;
        myS.str3.innerField3 := myS.str1.innerField3 + myS.str2.innerField3;

    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut main_data);
    assert_eq!(11, main_data.my_s.mys1.field1);
    assert_eq!(12, main_data.my_s.mys1.field2);
    assert_eq!(13, main_data.my_s.mys1.field3);

    assert_eq!(21, main_data.my_s.mys2.field1);
    assert_eq!(22, main_data.my_s.mys2.field2);
    assert_eq!(23, main_data.my_s.mys2.field3);

    assert_eq!(32, main_data.my_s.mys3.field1);
    assert_eq!(34, main_data.my_s.mys3.field2);
    assert_eq!(36, main_data.my_s.mys3.field3);
}

#[test]
fn using_enums() {
    #[repr(C)]
    struct ThreeFields {
        field1: i32,
        field2: i32,
        field3: i32,
    }
    let mut d = ThreeFields { field1: 0, field2: 0, field3: 0 };

    let testcode = r#"
    TYPE TrafficLight:
        (White, Red, Yellow, Green);
    END_TYPE

    PROGRAM main
    VAR
        tf1 : TrafficLight;        
        tf2 : TrafficLight;        
        tf3 : TrafficLight;        
    END_VAR
        tf1 := Red;
        tf2 := Yellow;
        tf3 := Green;
        
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut d);
    assert_eq!(1, d.field1);
    assert_eq!(2, d.field2);
    assert_eq!(3, d.field3);
}

#[test]
fn using_inline_enums() {
    #[repr(C)]
    struct ThreeFields {
        field1: i32,
        field2: i32,
        field3: i32,
    }
    let mut d = ThreeFields { field1: 0, field2: 0, field3: 0 };

    let testcode = r#"
    TYPE TrafficLight:
        (White, Red, Yellow, Green);
    END_TYPE

    PROGRAM main
    VAR
        tf1 : (White1, Red1, Yellow1, Green1);        
        tf2 : (White2, Red2, Yellow2, Green2);        
        tf3 : (White3, Red3, Yellow3, Green3);        
    END_VAR
        tf1 := Red1;
        tf2 := Yellow2;
        tf3 := Green3;
        
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut d);
    assert_eq!(1, d.field1);
    assert_eq!(2, d.field2);
    assert_eq!(3, d.field3);
}

#[test]
fn using_duplicate_enums_with_casts() {
    #[repr(C)]
    struct ThreeFields {
        field1: u8,
        field2: u16,
        field3: u32,
    }
    let mut d = ThreeFields { field1: 0, field2: 0, field3: 0 };

    let testcode = r#"
    TYPE MyEnum: BYTE(red := 1, yellow := 2, green := 3);
    END_TYPE

    TYPE MyEnum2: UINT(red := 10, yellow := 11, green := 12);
    END_TYPE
    
    TYPE MyEnum3: DINT(red := 22, yellow := 33, green := 44);
    END_TYPE


    PROGRAM main
    VAR
        tf1 : MyEnum;        
        tf2 : MyEnum2;        
        tf3 : MyEnum3;        
    END_VAR
        tf1 := MyEnum#red;
        tf2 := MyEnum2#yellow;
        tf3 := MyEnum3#green;
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut d);
    assert_eq!((1u8, 11u16, 44u32), (d.field1, d.field2, d.field3));
}

#[test]
fn using_inline_enums_in_structs() {
    #[repr(C)]
    struct MyStruct {
        tf1: i32,
        tf2: i32,
        tf3: i32,
    }
    let mut data = MyStruct { tf1: 0, tf2: 0, tf3: 0 };

    let testcode = r#"
    TYPE TrafficLight:
        (White, Red, Yellow, Green);
    END_TYPE
    
    TYPE MyStruct:
    STRUCT
        tf1 : TrafficLight;
        tf2 : TrafficLight;
        tf3 : TrafficLight;
    END_STRUCT
    END_TYPE

    PROGRAM main
    VAR
        data : MyStruct;
    END_VAR
        data.tf1 := Yellow;
        data.tf2 := Green;
        data.tf3 := data.tf1;
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut data);
    assert_eq!(2, data.tf1); //yellow
    assert_eq!(3, data.tf2); //green
    assert_eq!(2, data.tf3); //yellow
}

#[test]
fn using_inline_arrays_in_structs() {
    #[repr(C)]
    struct MyStruct {
        arr1: [i16; 4],
        arr2: [i16; 8],
        arr3: [i16; 3],
    }
    let mut data = MyStruct { arr1: [0; 4], arr2: [0; 8], arr3: [0; 3] };

    let testcode = r#"
    
    TYPE MyStruct:
    STRUCT
        arr1 : ARRAY[0..3] OF INT;
        arr2 : ARRAY[0..7] OF INT;
        arr3 : ARRAY[0..2] OF INT;
    END_STRUCT
    END_TYPE

    PROGRAM main
    VAR
        data : MyStruct;
        i : INT;
    END_VAR

    FOR i := 0 TO 3 DO
        data.arr1[i] := i;
        data.arr2[i] := i*10;
    END_FOR

    data.arr2[7] := 77;
    data.arr3[0] := data.arr2[7];
    data.arr3[2] := -1;

    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut data);
    assert_eq!([0, 1, 2, 3], data.arr1);
    assert_eq!([0, 10, 20, 30, 0, 0, 0, 77], data.arr2);
    assert_eq!([77, 0, -1], data.arr3);
}
#[test]
fn using_arrays() {
    #[warn(dead_code)]
    struct Main {
        arr: [i32; 10],
    }

    let mut main = Main { arr: [0; 10] };

    let testcode = r#"
    TYPE ARR : ARRAY[0..9] OF DINT; END_TYPE

    PROGRAM main
    VAR
        arr : ARR;
        i : INT;
    END_VAR
    FOR i := 0 TO 10 DO
        arr[i] := i;
    END_FOR
    END_PROGRAM
    "#;

    let _: i32 = compile_and_run(testcode, &mut main);
    for (i, j) in main.arr.iter_mut().enumerate() {
        assert_eq!(i as i32, *j);
    }
}
