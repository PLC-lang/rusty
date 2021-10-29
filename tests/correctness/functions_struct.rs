use super::super::*;
// Copyright (c) 2021 Daniel Schwenniger
use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct datatype_0 {
field_0: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct datatype_1 {
field_1: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct datatype_2 {
field_2: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct datatype_3 {
field_3: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct datatype_4 {
field_4: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeInit {
    local_var0: datatype_0,
    local_var1: datatype_1,
    local_var2: datatype_2,
    local_var3: datatype_3,
    local_var4: datatype_4,
    b_ret0: bool,
}

fn new() -> MainTypeInit {
  MainTypeInit::default()
}

#[test]
fn function_init() {
    let function = r"

    TYPE datatype_0 :
    STRUCT
          field_0 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_1 :
    STRUCT
      field_1 : INT := 0;
    END_STRUCT
     
    TYPE datatype_2 :
    STRUCT
      field_2 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_3 :
    STRUCT
        field_3 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_4 :
    STRUCT
      field_4 : INT := 0;
    END_STRUCT
    END_TYPE

    FUNCTION function_0 : BOOL
    VAR_INPUT
        in_var0: datatype_0 := (field_0 := 100);
        in_var1: datatype_1 := (field_1 := 200);
        in_var2: datatype_2 := (field_2 := 300);
        in_var3: datatype_3 := (field_3 := 400);
        in_var4: datatype_4 := (field_4 := 500); 
    END_VAR

      IF in_var0.field_0 = 100 OR in_var1.field_1 = 200 OR in_var2.field_2 = 300 OR in_var3.field_3 = 400 OR in_var4.field_4 = 500 THEN
          function_0 := TRUE;
      ELSIF in_var0.field_0 <> 10 OR in_var1.field_1 <> 20 OR in_var2.field_2 <> 30 OR in_var3.field_3 <> 40 OR in_var4.field_4 <> 50 THEN
          function_0 := TRUE;
      END_IF

      function_0 := FALSE;
    END_FUNCTION

    PROGRAM main
    VAR
        local_var0: datatype_0;
        local_var1: datatype_1;
        local_var2: datatype_2;
        local_var3: datatype_3;
        local_var4: datatype_4;
        b_ret0: BOOL;
    END_VAR

    local_var0.field_0 := 10;
    local_var1.field_1 := 20;
    local_var2.field_2 := 30;
    local_var4.field_4:= 40;

    b_ret0 := function_0(in_var0 := local_var0, in_var1 := local_var1, in_var2 := local_var2, in_var3 := local_var3, in_var4 := local_var4);
    END_PROGRAM
  ";

    let mut MainTypeInit = new();

    compile_and_run::<_, i32>(function.to_string(), &mut MainTypeInit);

    assert_eq!(true, MainTypeInit.b_ret0);

}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeStruct {
  local_var0: datatype_0,
  local_var1: datatype_1,
  local_var2: datatype_2,
  local_var3: datatype_3,
  local_var4: datatype_4,
   b_ret0: bool,
}

fn newWithStruct() -> MainTypeStruct {
    MainTypeStruct::default()
}

#[test]
fn function_call_struct() {
    let function = r"

    TYPE datatype_0 :
    STRUCT
          field_0 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_1 :
    STRUCT
      field_1 : INT := 0;
    END_STRUCT
    
    TYPE datatype_2 :
    STRUCT
      field_2 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_3 :
    STRUCT
        field_3 : INT := 0;
    END_STRUCT
    END_TYPE

    TYPE datatype_4 :
    STRUCT
      field_4 : INT := 0;
    END_STRUCT
    END_TYPE
 
    FUNCTION function_0 : BOOL
    VAR_INPUT
        in_var0: datatype_0;
        in_var1: datatype_1;
        in_var2: datatype_2;
        in_var3: datatype_3;
        in_var4: datatype_4; 
    END_VAR
    VAR
    END_VAR
    
    IF in_var0.field_0 <> 10 OR in_var1.field_1 <> 20 OR in_var2.field_2 <> 30 OR in_var3.field_3 <> 40 OR in_var4.field_4 <> 50 THEN
        function_0 := TRUE;
    END_IF;
    
    function_0 := FALSE;
    
    END_FUNCTION

    PROGRAM main
  VAR
    local_var0: datatype_0;
    local_var1: datatype_1;
    local_var2: datatype_2;
    local_var3: datatype_3;
    local_var4: datatype_4;
    
    b_ret0: BOOL;
  END_VAR

    local_var0.field_0 := 10;
    local_var1.field_1 := 20;
    local_var2.field_2 := 30;
    local_var3.field_3 := 40;
    local_var4.field_4:= 50;

    b_ret0 := function_0(in_var0 := local_var0, in_var1 := local_var1, in_var2 := local_var2, in_var3 := local_var3, in_var4 := local_var4);

END_PROGRAM
    ";

    let mut newWithStruct: MainTypeStruct = newWithStruct();

    compile_and_run::<_, i32>(function.to_string(), &mut newWithStruct);

    assert_eq!(false, newWithStruct.b_ret0);
}
