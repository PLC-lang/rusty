// Copyright (c) 2021 Daniel Schwenniger

use super::super::*;
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

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct  datatype_5 {
  a_start_point : [i16; 2],
  a_point1     : [i16; 2],
  a_point2     : [i16; 2],
  a_point3     : [i16; 2],
  a_point4     : [i16; 2],
  a_end_point   :[i16; 2],
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct function_block_0 {
  in_var0    : datatype_0,
  in_var1    : datatype_1,
  in_var2    : datatype_2,
  in_var3    : datatype_3,
  in_var4    : datatype_4,
  in_var5    : datatype_5,
  ret_val    : bool,
  out_var0    : datatype_0,
  out_var1    : datatype_1,
  out_var2    : datatype_2,
  out_var3    : datatype_3,
  out_var4    : datatype_4,
  out_var5    : datatype_5,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeFbInit {
  fb_0: function_block_0,
  b_ret0: bool,
  b_ret1: bool,
}

fn newWithFbInit() -> MainTypeFbInit {
  MainTypeFbInit::default()
}

#[test]
fn fb_init() {
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
 
    TYPE datatype_5 :
    STRUCT
      a_start_point : ARRAY [1..2] OF INT;
      a_point1     : ARRAY [1..2] OF INT;
      a_point2     : ARRAY [1..2] OF INT;
      a_point3     : ARRAY [1..2] OF INT;
      a_point4     : ARRAY [1..2] OF INT;
      a_end_point   : ARRAY [1..2] OF INT;		
    END_STRUCT
    END_TYPE

    FUNCTION_BLOCK function_block_0
    VAR_INPUT
        in_var0    : datatype_0 := (field_0 := 100);
        in_var1    : datatype_1 := (field_1 := 200);
        in_var2    : datatype_2 := (field_2 := 300);
        in_var3    : datatype_3 := (field_3 := 400);
        in_var4    : datatype_4 := (field_4 := 500);

        in_var5    : datatype_5 := (a_start_point := 3,3, a_point1 := 5,2, a_point2 := 7,3,  
                                    a_point3 := 8,5, a_point4 := 5,7, a_end_point:=3,5);
    END_VAR
    VAR_OUTPUT
        ret_val    : BOOL;
        out_var0    : datatype_0 := (field_0 := 10000);
        out_var1    : datatype_1 := (field_1 := 20000);
        out_var2    : datatype_2 := (field_2 := 30000);
        out_var3    : datatype_3 := (field_3 := 40000);
        out_var4    : datatype_4 := (field_4 := 50000);

        out_var5    : datatype_5 := (a_start_point := 3,3, a_point1 := 5,2, a_point2 := 7,3,  
                                    a_point3 := 8,5, a_point4 := 5,7, a_end_point:=3,5);
    END_VAR
    VAR

    END_VAR

    IF in_var0.field_0 <> 100 OR in_var1.field_1 <> 200 OR in_var2.field_2 <> 300 OR in_var3.field_3 <> 400 OR in_var4.field_4 <> 500 THEN
        ret_val := TRUE;
    ELSIF in_var5.a_start_point[1] <> 3 OR 
          in_var5.a_start_point[2] <> 3 OR 
          in_var5.a_point1[1] <> 5 OR 
          in_var5.a_point1[2] <> 2 OR 
          in_var5.a_point2[1] <> 7 OR 
          in_var5.a_point2[2] <> 3 OR 
          in_var5.a_point3[1] <> 8 OR 
          in_var5.a_point3[2] <> 5 OR 
          in_var5.a_point4[1] <> 5 OR 
          in_var5.a_point4[2] <> 7 OR 
          in_var5.a_end_point[1] <> 3 OR 
          in_var5.a_end_point[2] <> 5 THEN
        ret_val := TRUE;
    ELSIF out_var0.field_0 <> 10000 OR out_var1.field_1 <> 20000 OR out_var2.field_2 <> 30000 OR out_var3.field_3 <> 40000 OR out_var4.field_4 <> 50000 THEN
        ret_val := TRUE;
    ELSIF out_var5.a_start_point[1] <> 3 OR 
          out_var5.a_start_point[2] <> 3 OR 
          out_var5.a_point1[1] <> 5 OR 
          out_var5.a_point1[2] <> 2 OR 
          out_var5.a_point2[1] <> 7 OR 
          out_var5.a_point2[2] <> 3 OR 
          out_var5.a_point3[1] <> 8 OR 
          out_var5.a_point3[2] <> 5 OR 
          out_var5.a_point4[1] <> 5 OR 
          out_var5.a_point4[2] <> 7 OR 
          out_var5.a_end_point[1] <> 3 OR 
          out_var5.a_end_point[2] <> 5 THEN
        ret_val := TRUE;
    END_IF
    END_FUNCTION_BLOCK

    PROGRAM main
    VAR
        fb_0: function_block_0;

        b_ret0: BOOL;
        b_ret1: BOOL;
    END_VAR

    fb_0();

    b_ret0 := fb_0.ret_val;

    IF fb_0.out_var0.field_0 <> 10000 OR fb_0.out_var1.field_1 <> 20000 OR fb_0.out_var2.field_2 <> 30000 OR fb_0.out_var3.field_3 <> 40000 OR fb_0.out_var4.field_4 <> 50000 THEN
        b_ret1 := TRUE;
    ELSIF fb_0.out_var5.a_start_point[1] <> 3 OR 
          fb_0.out_var5.a_start_point[2] <> 3 OR 
          fb_0.out_var5.a_point1[1] <> 5 OR 
          fb_0.out_var5.a_point1[2] <> 2 OR 
          fb_0.out_var5.a_point2[1] <> 7 OR 
          fb_0.out_var5.a_point2[2] <> 3 OR 
          fb_0.out_var5.a_point3[1] <> 8 OR 
          fb_0.out_var5.a_point3[2] <> 5 OR 
          fb_0.out_var5.a_point4[1] <> 5 OR 
          fb_0.out_var5.a_point4[2] <> 7 OR 
          fb_0.out_var5.a_end_point[1] <> 3 OR 
          fb_0.out_var5.a_end_point[2] <> 5 THEN
        b_ret1 := TRUE;
    END_IF

    END_PROGRAM
   ";

    let mut newFbInit: MainTypeFbInit = newWithFbInit();

    compile_and_run::<_, i32>(function.to_string(), &mut newFbInit);

    assert_eq!(false, newFbInit.b_ret0);
    assert_eq!(false, newFbInit.b_ret1);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct fb_0 {
  in_var0    : datatype_0,
  in_var1    : datatype_1,
  in_var2    : datatype_2,
  in_var3    : datatype_3,
  in_var4    : datatype_4,
  in_var5    : datatype_5,
  in_out_var0    : datatype_0,
  in_out_var1    : datatype_1,
  in_out_var2    : datatype_2,
  in_out_var3    : datatype_3,
  in_out_var4    : datatype_4,
  in_out_var5    : datatype_5,
  out_var0    : datatype_0,
  out_var1    : datatype_1,
  out_var2    : datatype_2,
  out_var3    : datatype_3,
  out_var4    : datatype_4,
  out_var5    : datatype_5,
  b_ret_in_val    : bool,
  b_ret_in_out_val    : bool,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeCallFb {
  fb_0    : fb_0,
  in_var0    : datatype_0,
  in_var1    : datatype_1,
  in_var2    : datatype_2,
  in_var3    : datatype_3,
  in_var4    : datatype_4,
  in_var5    : datatype_5,
  in_out_var0    : datatype_0,
  in_out_var1    : datatype_1,
  in_out_var2    : datatype_2,
  in_out_var3    : datatype_3,
  in_out_var4    : datatype_4,
  in_out_var5    : datatype_5,
  out_var0    : datatype_0,
  out_var1    : datatype_1,
  out_var2    : datatype_2,
  out_var3    : datatype_3,
  out_var4    : datatype_4,
  out_var5    : datatype_5,
  b_ret_in_val    : bool,
  b_ret_in_out_val    : bool,
  b_ret_out_val    : bool,
}

fn newWithFbCall() -> MainTypeCallFb {
  MainTypeCallFb::default()
}

#[test]
fn fb_call() {
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
      END_TYPE
      
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
  
      TYPE datatype_5 :
      STRUCT
        a_start_point : ARRAY [1..2] OF INT;
        a_point1     : ARRAY [1..2] OF INT;
        a_point2     : ARRAY [1..2] OF INT;
        a_point3     : ARRAY [1..2] OF INT;
        a_point4     : ARRAY [1..2] OF INT;
        a_end_point   : ARRAY [1..2] OF INT;		
      END_STRUCT
      END_TYPE

      FUNCTION_BLOCK fb_0
      VAR_INPUT
        in_var0    : datatype_0;
        in_var1    : datatype_1;
        in_var2    : datatype_2;
        in_var3    : datatype_3;
        in_var4    : datatype_4;
        in_var5    : datatype_5;
    END_VAR
    VAR_IN_OUT
        in_out_var0    : datatype_0;
        in_out_var1    : datatype_1;
        in_out_var2    : datatype_2;
        in_out_var3    : datatype_3;
        in_out_var4    : datatype_4;
        in_out_var5    : datatype_5;
    END_VAR
  VAR_OUTPUT
      out_var0    : datatype_0;
      out_var1    : datatype_1;
      out_var2    : datatype_2;
      out_var3    : datatype_3;
      out_var4    : datatype_4;
      out_var5    : datatype_5;
      b_ret_in_val    : BOOL;
      b_ret_in_out_val    : BOOL;
  END_VAR

  IF in_var0.field_0 <> 10 OR in_var1.field_1 <> 20 OR in_var2.field_2 <> 30 OR in_var3.field_3 <> 40 OR in_var4.field_4 <> 50 THEN
      b_ret_in_val := TRUE;
  ELSIF in_var5.a_start_point[1] <> 60 OR 
        in_var5.a_start_point[2] <> 70 OR 
        in_var5.a_point1[1] <> 80 OR 
        in_var5.a_point1[2] <> 90 OR 
        in_var5.a_point2[1] <> 100 OR 
        in_var5.a_point2[2] <> 110 OR 
        in_var5.a_point3[1] <> 120 OR 
        in_var5.a_point3[2] <> 130 OR 
        in_var5.a_point4[1] <> 140 OR 
        in_var5.a_point4[2] <> 150 OR 
        in_var5.a_end_point[1] <> 160 OR 
        in_var5.a_end_point[2] <> 170 THEN
          b_ret_in_val := FALSE;
  END_IF

  IF in_out_var0.field_0 <> 10 OR in_out_var1.field_1 <> 20 OR in_out_var2.field_2 <> 30 OR in_out_var3.field_3 <> 40 OR in_out_var4.field_4 <> 50 THEN
      b_ret_in_out_val := TRUE;
  ELSIF in_out_var5.a_start_point[1] <> 60 OR 
        in_out_var5.a_start_point[2] <> 70 OR 
        in_out_var5.a_point1[1] <> 80 OR 
        in_out_var5.a_point1[2] <> 90 OR 
        in_out_var5.a_point2[1] <> 100 OR 
        in_out_var5.a_point2[2] <> 110 OR 
        in_out_var5.a_point3[1] <> 120 OR 
        in_out_var5.a_point3[2] <> 130 OR 
        in_out_var5.a_point4[1] <> 140 OR 
        in_out_var5.a_point4[2] <> 150 OR 
        in_out_var5.a_end_point[1] <> 160 OR 
        in_out_var5.a_end_point[2] <> 170 THEN
          b_ret_in_out_val := FALSE;
  END_IF

  in_out_var0.field_0 := in_out_var0.field_0 + 1;
  in_out_var1.field_1 := in_out_var1.field_1 + 1;
  in_out_var2.field_2 := in_out_var2.field_2 + 1;
  in_out_var3.field_3 := in_out_var3.field_3 + 1;
  in_out_var4.field_4 := in_out_var4.field_4 + 1;
  in_out_var5.a_start_point[1] := in_out_var5.a_start_point[1] + 1;
  in_out_var5.a_start_point[2] := in_out_var5.a_start_point[2] + 1;
  in_out_var5.a_point1[1] := in_out_var5.a_point1[1] + 1;
  in_out_var5.a_point1[2] := in_out_var5.a_point1[2] + 1;
  in_out_var5.a_point2[1] := in_out_var5.a_point2[1] + 1;
  in_out_var5.a_point2[2] := in_out_var5.a_point2[2] + 1;
  in_out_var5.a_point3[1] := in_out_var5.a_point3[1] + 1;
  in_out_var5.a_point3[2] := in_out_var5.a_point3[2] + 1;
  in_out_var5.a_point4[1] := in_out_var5.a_point4[1] + 1;
  in_out_var5.a_point4[2] := in_out_var5.a_point4[2] + 1;
  in_out_var5.a_end_point[1] := in_out_var5.a_end_point[1] + 1;
  in_out_var5.a_end_point[2] := in_out_var5.a_end_point[2] + 1;

  out_var0.field_0 := 100;
  out_var1.field_1 := 200;
  out_var2.field_2 := 300;
  out_var3.field_3 := 400;
  out_var4.field_4 := 500;
  out_var5.a_start_point[1] := 600;
  out_var5.a_start_point[2] := 700;
  out_var5.a_point1[1] := 800;
  out_var5.a_point1[2] := 900;
  out_var5.a_point2[1] := 1000;
  out_var5.a_point2[2] := 1100;
  out_var5.a_point3[1] := 1200;
  out_var5.a_point3[2] := 1300;
  out_var5.a_point4[1] := 1400;
  out_var5.a_point4[2] := 1500;
  out_var5.a_end_point[1] := 1600;
  out_var5.a_end_point[2] := 1700;

  END_FUNCTION_BLOCK

  PROGRAM main
  VAR
    fb_0    : fb_0;
    in_var0    : datatype_0;
    in_var1    : datatype_1;
    in_var2    : datatype_2;
    in_var3    : datatype_3;
    in_var4    : datatype_4;
    in_var5    : datatype_5;
    in_out_var0    : datatype_0;
    in_out_var1    : datatype_1;
    in_out_var2    : datatype_2;
    in_out_var3    : datatype_3;
    in_out_var4    : datatype_4;
    in_out_var5    : datatype_5;
    out_var0    : datatype_0;
    out_var1    : datatype_1;
    out_var2    : datatype_2;
    out_var3    : datatype_3;
    out_var4    : datatype_4;
    out_var5    : datatype_5;
    b_ret_in_val    : BOOL;
    b_ret_in_out_val    : BOOL;
    b_ret_out_val    : BOOL;
  END_VAR
  
  in_var0.field_0 := 10;
  in_var1.field_1 := 20;
  in_var2.field_2 := 30;
  in_var3.field_3 := 40;
  in_var4.field_4 := 50;
  in_var5.a_start_point[1] := 60;
  in_var5.a_start_point[2] := 70;
  in_var5.a_point1[1] := 80;
  in_var5.a_point1[2] := 90;
  in_var5.a_point2[1] := 100;
  in_var5.a_point2[2] := 110;
  in_var5.a_point3[1] := 120;
  in_var5.a_point3[2] := 130;
  in_var5.a_point4[1] := 140;
  in_var5.a_point4[2] := 150;
  in_var5.a_end_point[1] := 160;
  in_var5.a_end_point[2] := 170;
  
  in_out_var0.field_0 := 10;
  in_out_var1.field_1 := 20;
  in_out_var2.field_2 := 30;
  in_out_var3.field_3 := 40;
  in_out_var4.field_4 := 50;
  in_out_var5.a_start_point[1] := 60;
  in_out_var5.a_start_point[2] := 70;
  in_out_var5.a_point1[1] := 80;
  in_out_var5.a_point1[2] := 90;
  in_out_var5.a_point2[1] := 100;
  in_out_var5.a_point2[2] := 110;
  in_out_var5.a_point3[1] := 120;
  in_out_var5.a_point3[2] := 130;
  in_out_var5.a_point4[1] := 140;
  in_out_var5.a_point4[2] := 150;
  in_out_var5.a_end_point[1] := 160;
  in_out_var5.a_end_point[2] := 170;
  
  fb_0(
      in_var0 := in_var0,
      in_var1 := in_var1,
      in_var2 := in_var2,
      in_var3 := in_var3,
      in_var4 := in_var4,
      in_var5 := in_var5,
      in_out_var0 := in_out_var0,
      in_out_var1 := in_out_var1,
      in_out_var2 := in_out_var2,
      in_out_var3 := in_out_var3,
      in_out_var4 := in_out_var4,
      in_out_var5 := in_out_var5,
      out_var0 => out_var0,
      out_var1 => out_var1,
      out_var2 => out_var2,
      out_var3 => out_var3,
      out_var4 => out_var4,
      out_var5 => out_var5,
      b_ret_in_val => b_ret_in_val,
      b_ret_in_out_val => b_ret_in_out_val);
  
  IF in_out_var0.field_0 <> 11 OR in_out_var1.field_1 <> 21 OR in_out_var2.field_2 <> 31 OR in_out_var3.field_3 <> 41 OR in_out_var4.field_4 <> 51 THEN
      b_ret_in_out_val := TRUE;
  ELSIF in_out_var5.a_start_point[1] <> 61 OR 
        in_out_var5.a_start_point[2] <> 71 OR 
        in_out_var5.a_point1[1] <> 81 OR 
        in_out_var5.a_point1[2] <> 91 OR 
        in_out_var5.a_point2[1] <> 101 OR 
        in_out_var5.a_point2[2] <> 111 OR 
        in_out_var5.a_point3[1] <> 121 OR 
        in_out_var5.a_point3[2] <> 131 OR 
        in_out_var5.a_point4[1] <> 141 OR 
        in_out_var5.a_point4[2] <> 151 OR
        in_out_var5.a_end_point[1] <> 161 OR
        in_out_var5.a_end_point[2] <> 171 THEN
      b_ret_in_out_val := TRUE;
  END_IF
  
  IF out_var0.field_0 <> 100 OR out_var1.field_1 <> 200 OR fb_0.out_var2.field_2 <> 300 OR fb_0.out_var3.field_3 <> 400 OR fb_0.out_var4.field_4 <> 500 THEN
      b_ret_out_val := TRUE;
  ELSIF out_var5.a_start_point[1] <> 600 OR 
        out_var5.a_start_point[2] <> 700 OR 
        out_var5.a_point1[1] <> 800 OR 
        fb_0.out_var5.a_point1[2] <> 900 OR 
        fb_0.out_var5.a_point2[1] <> 1000 OR 
        fb_0.out_var5.a_point2[2] <> 1100 OR 
        fb_0.out_var5.a_point3[1] <> 1200 OR 
        fb_0.out_var5.a_point3[2] <> 1300 OR 
        fb_0.out_var5.a_point4[1] <> 1400 OR 
        fb_0.out_var5.a_point4[2] <> 1500 OR
        fb_0.out_var5.a_end_point[1] <> 1600 OR
        fb_0.out_var5.a_end_point[2] <> 1700 THEN
      b_ret_out_val := TRUE;
  END_IF
  END_PROGRAM
      ";
      
      let mut newFbCall: MainTypeCallFb = newWithFbCall();
      
      compile_and_run::<_, i32>(function.to_string(), &mut newFbCall);
      
      assert_eq!(false, newFbCall.b_ret_in_val);
      assert_eq!(false, newFbCall.b_ret_in_out_val);
      assert_eq!(false, newFbCall.b_ret_out_val);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypePrgInit {
  b_ret0    : bool,
  b_ret1    : bool,
}

fn newWIthPrgInit() -> MainTypePrgInit {
  MainTypePrgInit::default()
}

#[test]
fn prg_init() {
    let function = r"

      TYPE datatype_1 :
      STRUCT
        field_1 : INT := 0;
      END_STRUCT
      END_TYPE
      
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
  
      TYPE datatype_5 :
      STRUCT
        a_start_point : ARRAY [1..2] OF INT;
        a_point1     : ARRAY [1..2] OF INT;
        a_point2     : ARRAY [1..2] OF INT;
        a_point3     : ARRAY [1..2] OF INT;
        a_point4     : ARRAY [1..2] OF INT;
        a_end_point   : ARRAY [1..2] OF INT;		
      END_STRUCT
      END_TYPE

      PROGRAM program_0
      VAR_INPUT
          in_var1    : datatype_1 := (field_1 := 200);
          in_var2    : datatype_2 := (field_2 := 300);
          in_var3    : datatype_3 := (field_3 := 400);
          in_var4    : datatype_4 := (field_4 := 500);
          in_var5    : datatype_5 := (a_start_point := 3,3, a_point1 := 5,2, a_point2 := 7,3,  
                                      a_point3 := 8,5, a_point4 := 5,7, a_end_point:=3,5);
      END_VAR
      VAR_OUTPUT
          out_var1    : datatype_1 := (field_1 := 20000);
          out_var2    : datatype_2 := (field_2 := 30000);
          out_var3    : datatype_3 := (field_3 := 40000);
          out_var4    : datatype_4 := (field_4 := 50000);
          out_var5    : datatype_5 := (a_start_point := 3,3, a_point1 := 5,2, a_point2 := 7,3,  
                                        a_point3 := 8,5, a_point4 := 5,7, a_end_point:=3,5);
          b_ret_val    : BOOL;
      END_VAR
      
      IF in_var1.field_1 <> 200 OR in_var2.field_2 <> 300 OR in_var3.field_3 <> 400 OR in_var4.field_4 <> 500 THEN
          b_ret_val := TRUE;
      ELSIF in_var5.a_start_point[1] <> 3 OR 
            in_var5.a_start_point[2] <> 3 OR 
            in_var5.a_point1[1] <> 5 OR 
            in_var5.a_point1[2] <> 2 OR 
            in_var5.a_point2[1] <> 7 OR 
            in_var5.a_point2[2] <> 3 OR 
            in_var5.a_point3[1] <> 8 OR 
            in_var5.a_point3[2] <> 5 OR 
            in_var5.a_point4[1] <> 5 OR 
            in_var5.a_point4[2] <> 7 OR 
            in_var5.a_end_point[1] <> 3 OR 
            in_var5.a_end_point[2] <> 5 THEN
          b_ret_val := TRUE;
      ELSIF out_var1.field_1 <> 20000 OR out_var2.field_2 <> 30000 OR out_var3.field_3 <> 40000 OR out_var4.field_4 <> 50000 THEN
          b_ret_val := TRUE;
      ELSIF out_var5.a_start_point[1] <> 3 OR 
            out_var5.a_start_point[2] <> 3 OR 
            out_var5.a_point1[1] <> 5 OR 
            out_var5.a_point1[2] <> 2 OR 
            out_var5.a_point2[1] <> 7 OR 
            out_var5.a_point2[2] <> 3 OR 
            out_var5.a_point3[1] <> 8 OR 
            out_var5.a_point3[2] <> 5 OR 
            out_var5.a_point4[1] <> 5 OR 
            out_var5.a_point4[2] <> 7 OR 
            out_var5.a_end_point[1] <> 3 OR 
            out_var5.a_end_point[2] <> 5 THEN
          b_ret_val := TRUE;
      END_IF
      
      END_PROGRAM

      PROGRAM main
      VAR
        b_ret0    : BOOL;
        b_ret1    : BOOL;
    END_VAR

    program_0();

    b_ret0 := program_0.b_ret_val;

    IF program_0.out_var1.field_1 <> 20000 OR program_0.out_var2.field_2 <> 30000 OR program_0.out_var3.field_3 <> 40000 OR program_0.out_var4.field_4 <> 50000 THEN
        b_ret1 := TRUE;
    ELSIF program_0.out_var5.a_start_point[1] <> 3 OR 
          program_0.out_var5.a_start_point[2] <> 3 OR 
          program_0.out_var5.a_point1[1] <> 5 OR 
          program_0.out_var5.a_point1[2] <> 2 OR 
          program_0.out_var5.a_point2[1] <> 7 OR 
          program_0.out_var5.a_point2[2] <> 3 OR 
          program_0.out_var5.a_point3[1] <> 8 OR 
          program_0.out_var5.a_point3[2] <> 5 OR 
          program_0.out_var5.a_point4[1] <> 5 OR 
          program_0.out_var5.a_point4[2] <> 7 OR 
          program_0.out_var5.a_end_point[1] <> 3 OR 
          program_0.out_var5.a_end_point[2] <> 5 THEN
        b_ret1 := TRUE;
    END_IF


    END_PROGRAM
      ";
      
      let mut newPrgInit: MainTypePrgInit = newWIthPrgInit();
      
      compile_and_run::<_, i32>(function.to_string(), &mut newPrgInit);
      
      assert_eq!(false, newPrgInit.b_ret0);
      assert_eq!(false, newPrgInit.b_ret1);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypePrgCall {
  b_ret0    : bool,
  b_ret1    : bool,
}

fn newWIthPrgCall() -> MainTypePrgCall {
  MainTypePrgCall::default()
}

#[test]
fn prg_call() {
    let function = r"

      TYPE datatype_1 :
      STRUCT
        field_1 : INT := 0;
      END_STRUCT
      END_TYPE
      
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
  
      TYPE datatype_5 :
      STRUCT
        a_start_point : ARRAY [1..2] OF INT;
        a_point1     : ARRAY [1..2] OF INT;
        a_point2     : ARRAY [1..2] OF INT;
        a_point3     : ARRAY [1..2] OF INT;
        a_point4     : ARRAY [1..2] OF INT;
        a_end_point   : ARRAY [1..2] OF INT;		
      END_STRUCT
      END_TYPE

      PROGRAM program_1
      VAR_INPUT
          in_var1    : datatype_1;
          in_var2    : datatype_2;
          in_var3    : datatype_3;
          in_var4    : datatype_4;
          in_var5    : datatype_5;
      END_VAR
      VAR_IN_OUT
          in_out_var1    : datatype_1;
          in_out_var2    : datatype_2;
          in_out_var3    : datatype_3;
          in_out_var4    : datatype_4;
          in_out_var5    : datatype_5;
      END_VAR
      VAR_OUTPUT
          out_var1    : datatype_1;
          out_var2    : datatype_2;
          out_var3    : datatype_3;
          out_var4    : datatype_4;
          out_var5    : datatype_5;
          b_ret_in_val    : BOOL;
          b_ret_in_out_val    : BOOL;
      END_VAR
      VAR
      END_VAR

      IF in_var1.field_1 <> 20 OR in_var2.field_2 <> 30 OR in_var3.field_3 <> 40 OR in_var4.field_4 <> 50 THEN
          b_ret_in_val := TRUE;
      ELSIF in_var5.a_start_point[1] <> 60 OR 
            in_var5.a_start_point[2] <> 70 OR 
            in_var5.a_point1[1] <> 80 OR 
            in_var5.a_point1[2] <> 90 OR 
            in_var5.a_point2[1] <> 100 OR 
            in_var5.a_point2[2] <> 110 OR 
            in_var5.a_point3[1] <> 120 OR 
            in_var5.a_point3[2] <> 130 OR 
            in_var5.a_point4[1] <> 140 OR 
            in_var5.a_point4[2] <> 150 OR 
            in_var5.a_end_point[1] <> 160 OR 
            in_var5.a_end_point[2] <> 170 THEN
          b_ret_in_val := FALSE;
      END_IF

      IF in_out_var1.field_1 <> 20 OR in_out_var2.field_2 <> 30 OR in_out_var3.field_3 <> 40 OR in_out_var4.field_4 <> 50 THEN
          b_ret_in_out_val := TRUE;
      ELSIF in_out_var5.a_start_point[1] <> 60 OR 
            in_out_var5.a_start_point[2] <> 70 OR 
            in_out_var5.a_point1[1] <> 80 OR 
            in_out_var5.a_point1[2] <> 90 OR 
            in_out_var5.a_point2[1] <> 100 OR 
            in_out_var5.a_point2[2] <> 110 OR 
            in_out_var5.a_point3[1] <> 120 OR 
            in_out_var5.a_point3[2] <> 130 OR 
            in_out_var5.a_point4[1] <> 140 OR 
            in_out_var5.a_point4[2] <> 150 OR 
            in_out_var5.a_end_point[1] <> 160 OR 
            in_out_var5.a_end_point[2] <> 170 THEN
          b_ret_in_out_val := FALSE;
      END_IF

      in_out_var1.field_1 := in_out_var1.field_1 + 1;
      in_out_var2.field_2 := in_out_var2.field_2 + 1;
      in_out_var3.field_3 := in_out_var3.field_3 + 1;
      in_out_var4.field_4 := in_out_var4.field_4 + 1;
      in_out_var5.a_start_point[1] := in_out_var5.a_start_point[1] + 1;
      in_out_var5.a_start_point[2] := in_out_var5.a_start_point[2] + 1;
      in_out_var5.a_point1[1] := in_out_var5.a_point1[1] + 1;
      in_out_var5.a_point1[2] := in_out_var5.a_point1[2] + 1;
      in_out_var5.a_point2[1] := in_out_var5.a_point2[1] + 1;
      in_out_var5.a_point2[2] := in_out_var5.a_point2[2] + 1;
      in_out_var5.a_point3[1] := in_out_var5.a_point3[1] + 1;
      in_out_var5.a_point3[2] := in_out_var5.a_point3[2] + 1;
      in_out_var5.a_point4[1] := in_out_var5.a_point4[1] + 1;
      in_out_var5.a_point4[2] := in_out_var5.a_point4[2] + 1;
      in_out_var5.a_end_point[1] := in_out_var5.a_end_point[1] + 1;
      in_out_var5.a_end_point[2] := in_out_var5.a_end_point[2] + 1;

      out_var1.field_1 := 200;
      out_var2.field_2 := 300;
      out_var3.field_3 := 400;
      out_var4.field_4 := 500;
      out_var5.a_start_point[1] := 600;
      out_var5.a_start_point[2] := 700;
      out_var5.a_point1[1] := 800;
      out_var5.a_point1[2] := 900;
      out_var5.a_point2[1] := 1000;
      out_var5.a_point2[2] := 1100;
      out_var5.a_point3[1] := 1200;
      out_var5.a_point3[2] := 1300;
      out_var5.a_point4[1] := 1400;
      out_var5.a_point4[2] := 1500;
      out_var5.a_end_point[1] := 1600;
      out_var5.a_end_point[2] := 1700;
      END_PROGRAM

      PROGRAM main
      VAR
        in_var1    : datatype_1;
        in_var2    : datatype_2;
        in_var3    : datatype_3;
        in_var4    : datatype_4;
        in_var5    : datatype_5;
        in_out_var1    : datatype_1;
        in_out_var2    : datatype_2;
        in_out_var3    : datatype_3;
        in_out_var4    : datatype_4;
        in_out_var5    : datatype_5;
        out_var1    : datatype_1;
        out_var2    : datatype_2;
        out_var3    : datatype_3;
        out_var4    : datatype_4;
        out_var5    : datatype_5;
        b_ret_in_val    : BOOL;
        b_ret_in_out_val    : BOOL;
        b_ret_out_val    : BOOL;
    END_VAR
  
    in_var1.field_1 := 20;
    in_var2.field_2 := 30;
    in_var3.field_3 := 40;
    in_var4.field_4 := 50;
    in_var5.a_start_point[1] := 60;
    in_var5.a_start_point[2] := 70;
    in_var5.a_point1[1] := 80;
    in_var5.a_point1[2] := 90;
    in_var5.a_point2[1] := 100;
    in_var5.a_point2[2] := 110;
    in_var5.a_point3[1] := 120;
    in_var5.a_point3[2] := 130;
    in_var5.a_point4[1] := 140;
    in_var5.a_point4[2] := 150;
    in_var5.a_end_point[1] := 160;
    in_var5.a_end_point[2] := 170;
    
    in_out_var1.field_1 := 20;
    in_out_var2.field_2 := 30;
    in_out_var3.field_3 := 40;
    in_out_var4.field_4 := 50;
    in_out_var5.a_start_point[1] := 60;
    in_out_var5.a_start_point[2] := 70;
    in_out_var5.a_point1[1] := 80;
    in_out_var5.a_point1[2] := 90;
    in_out_var5.a_point2[1] := 100;
    in_out_var5.a_point2[2] := 110;
    in_out_var5.a_point3[1] := 120;
    in_out_var5.a_point3[2] := 130;
    in_out_var5.a_point4[1] := 140;
    in_out_var5.a_point4[2] := 150;
    in_out_var5.a_end_point[1] := 160;
    in_out_var5.a_end_point[2] := 170;
    
    program_1(
        in_var1 := in_var1,
        in_var2 := in_var2,
        in_var3 := in_var3,
        in_var4 := in_var4,
        in_var5 := in_var5,
        in_out_var1 := in_out_var1,
        in_out_var2 := in_out_var2,
        in_out_var3 := in_out_var3,
        in_out_var4 := in_out_var4,
        in_out_var5 := in_out_var5,
        out_var1 => out_var1,
        out_var2 => out_var2,
        out_var3 => out_var3,
        out_var4 => out_var4,
        out_var5 => out_var5,
        b_ret_in_val => b_ret_in_val,
        b_ret_in_out_val => b_ret_in_out_val);
    
    IF in_out_var1.field_1 <> 21 OR in_out_var2.field_2 <> 31 OR in_out_var3.field_3 <> 41 OR in_out_var4.field_4 <> 51 THEN
        b_ret_in_out_val := TRUE;
    ELSIF in_out_var5.a_start_point[1] <> 61 OR 
          in_out_var5.a_start_point[2] <> 71 OR 
          in_out_var5.a_point1[1] <> 81 OR 
          in_out_var5.a_point1[2] <> 91 OR 
          in_out_var5.a_point2[1] <> 101 OR 
          in_out_var5.a_point2[2] <> 111 OR 
          in_out_var5.a_point3[1] <> 121 OR 
          in_out_var5.a_point3[2] <> 131 OR 
          in_out_var5.a_point4[1] <> 141 OR 
          in_out_var5.a_point4[2] <> 151 OR
          in_out_var5.a_end_point[1] <> 161 OR
          in_out_var5.a_end_point[2] <> 171 THEN
        b_ret_in_out_val := TRUE;
    END_IF
    
    IF out_var1.field_1 <> 200 OR out_var2.field_2 <> 300 OR out_var3.field_3 <> 400 OR out_var4.field_4 <> 500 THEN
        b_ret_out_val := TRUE;
    ELSIF out_var5.a_start_point[1] <> 600 OR 
          out_var5.a_start_point[2] <> 700 OR 
          out_var5.a_point1[1] <> 800 OR 
          out_var5.a_point1[2] <> 900 OR 
          out_var5.a_point2[1] <> 1000 OR 
          out_var5.a_point2[2] <> 1100 OR 
          out_var5.a_point3[1] <> 1200 OR 
          out_var5.a_point3[2] <> 1300 OR 
          out_var5.a_point4[1] <> 1400 OR 
          out_var5.a_point4[2] <> 1500 OR
          out_var5.a_end_point[1] <> 1600 OR
          out_var5.a_end_point[2] <> 1700 THEN
        b_ret_out_val := TRUE;
    END_IF
  
    END_PROGRAM
      ";
      
      let mut newPrgCall: MainTypePrgCall = newWIthPrgCall();
      
      compile_and_run::<_, i32>(function.to_string(), &mut newPrgCall);
      
      assert_eq!(false, newPrgCall.b_ret0);
      assert_eq!(false, newPrgCall.b_ret1);
}