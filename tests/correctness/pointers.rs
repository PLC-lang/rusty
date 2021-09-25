// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
struct MainType {}

#[test]
fn pointer_test() {
    let function = r"
TYPE MyStruct: STRUCT  x: DINT; y: DINT; END_STRUCT END_TYPE
TYPE MyRef : REF_TO REF_TO DINT; END_TYPE;

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
u := &s.x;
y := u;
z := &y;
s.x := 9;
z^^ := y^*2;
v := z;
y^ := v^^*2;

foo := y^;
END_FUNCTION
 ";

    let mut maintype = MainType {};

    let res: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(36, res);
}
