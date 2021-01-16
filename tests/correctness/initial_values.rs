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
            z   : REAL := 3.1415;
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
        assert_eq!(3.1415, maintype.z);
        assert_eq!(0.0, maintype.z_);
}