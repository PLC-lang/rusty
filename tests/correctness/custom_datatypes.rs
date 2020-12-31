
use super::super::*;

#[test]
fn using_structs() {

    #[repr(C)]
    #[derive(Debug)]
    struct MyStruct {
        field1: i8,
        field2: i16,
        field3: i32,
    }

    let my_s = MyStruct {
        field1: 0,
        field2: 0,
        field3: 0,
    };

    struct MainType {
        my_s : MyStruct,
    }

    let mut main_data = MainType { my_s, };

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

    compile_and_run(testcode.to_string(), &mut main_data);
    assert_eq!(3, main_data.my_s.field1);
    assert_eq!(7, main_data.my_s.field2);
    assert_eq!(10, main_data.my_s.field3);
}


#[test]
fn using_enums() {
    struct ThreeFields {
        field1: i32,
        field2: i32,
        field3: i32,
    }
    let mut d = ThreeFields {
        field1: 0,
        field2: 0,
        field3: 0,
    };
    
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

    compile_and_run(testcode.to_string(), &mut d);
    assert_eq!(1, d.field1);
    assert_eq!(2, d.field2);
    assert_eq!(3, d.field3);
}