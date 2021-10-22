ModuleID = \'main\'
source_filename = \"main\"

%main_interface = type { [81 x i8], [81 x i8], [81 x i8] }
%read_string_interface = type { [81 x i8] }

@main_instance = global %main_interface zeroinitializer

define [81 x i8] @read_string(%read_string_interface* %0) {
entry:
  %to_read = getelementptr inbounds %read_string_interface, %read_string_interface* %0, i32 0, i32 0
  %read_string = alloca [81 x i8], align 1
  %load_to_read = load [81 x i8], [81 x i8]* %to_read, align 1
  store [81 x i8] %load_to_read, [81 x i8]* %read_string, align 1
  %read_string_ret = load [81 x i8], [81 x i8]* %read_string, align 1
  ret [81 x i8] %read_string_ret
}

define void @main(%main_interface* %0) {
entry:
  %text1 = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
  %text2 = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 1
  %text3 = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 2
  %read_string_instance = alloca %read_string_interface, align 8
  br label %input

input:                                            ; preds = %entry
  %1 = getelementptr inbounds %read_string_interface, %read_string_interface* %read_string_instance, i32 0, i32 0
  store [154 x i8] c\"abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\\00\", [81 x i8]* %1, align 1
  br label %call

call:                                             ; preds = %input
  %call1 = call [81 x i8] @read_string(%read_string_interface* %read_string_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store [81 x i8] %call1, [81 x i8]* %text1, align 1
  %read_string_instance2 = alloca %read_string_interface, align 8
  br label %input3

input3:                                           ; preds = %continue
  %2 = getelementptr inbounds %read_string_interface, %read_string_interface* %read_string_instance2, i32 0, i32 0
  store [6 x i8] c\"hello\\00\", [81 x i8]* %2, align 1
  br label %call4

call4:                                            ; preds = %input3
  %call7 = call [81 x i8] @read_string(%read_string_interface* %read_string_instance2)
  br label %output5

output5:                                          ; preds = %call4
  br label %continue6

continue6:                                        ; preds = %output5
  store [81 x i8] %call7, [81 x i8]* %text3, align 1
  ret void
}
