FUNCTION test_function : BOOL
VAR_INPUT
    input_data : struct_array;
END_VAR
VAR_IN_OUT  
    output_data : struct_array;
END_VAR
    test_function := TRUE;
END_FUNCTION

FUNCTION main : DINT
VAR
    // Array of typedef of struct
    my_struct_arr : struct_array;
    
    // Array of typedef of array  
    nested_arr : ARRAY[0..1] OF int_array;
    
    // Pointer to typedef of struct
    struct_pointer : struct_ptr;
    
    // Pointer to typedef of array
    array_pointer : array_ptr;
    
    // Complex container
    container : container_struct;
    
    // Multi-dimensional
    matrix : matrix_type;
    
    // Deep typedef chain
    deep_var : level2_typedef;
END_VAR
    // Test array of typedef of struct
    my_struct_arr[0].x := 1337;
    my_struct_arr[0].y := 3.14;
    
    // Test deep typedef chain  
    deep_var.x := 7331;
    deep_var.y := 2.71;
    
    // Test container with complex types
    container.arr_field[0].x := 9001;
    container.status := status_enum#RUNNING;
    container.percent := 75;
    
    // Test multi-dimensional arrays
    matrix[0][0] := 1.23;
    matrix[1][1] := 4.56;
    
    // Test function call
    test_function(my_struct_arr, my_struct_arr);
    
    printf('%d$N', my_struct_arr[0].x);     // CHECK: 1337
    printf('%.2f$N', my_struct_arr[0].y);   // CHECK: 3.14
    printf('%d$N', deep_var.x);             // CHECK: 7331
    printf('%.2f$N', deep_var.y);           // CHECK: 2.71
    printf('%d$N', container.arr_field[0].x); // CHECK: 9001
    printf('%d$N', container.status);       // CHECK: 1
    printf('%d$N', container.percent);      // CHECK: 75
    printf('%.2f$N', matrix[0][0]);         // CHECK: 1.23
    printf('%.2f$N', matrix[1][1]);         // CHECK: 4.56
    
    main := 0;
END_FUNCTION