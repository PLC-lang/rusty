"; ModuleID = 'main'
source_filename = \"main\"

%smaller_than_ten_interface = type { i8 }

define i16 @smaller_than_ten(%smaller_than_ten_interface* %0) {
entry:
  %n = getelementptr inbounds %smaller_than_ten_interface, %smaller_than_ten_interface* %0, i32 0, i32 0
  %smaller_than_ten = alloca i16, align 2
  %load_n = load i8, i8* %n, align 1
  %1 = sext i8 %load_n to i32
  %tmpVar = icmp slt i32 %1, 10
  br i1 %tmpVar, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  %smaller_than_ten_ret = load i16, i16* %smaller_than_ten, align 2
  ret i16 %smaller_than_ten_ret

buffer_block:                                     ; No predecessors!
  br label %continue

continue:                                         ; preds = %buffer_block, %entry
  %smaller_than_ten_ret1 = load i16, i16* %smaller_than_ten, align 2
  ret i16 %smaller_than_ten_ret1
}
