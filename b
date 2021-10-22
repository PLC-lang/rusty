; ModuleID = 'main'
source_filename = \"main\"

%foo_interface = type { i32* }
%prg_interface = type { i32 }

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %inout = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %deref = load i32*, i32** %inout, align 8
  %deref1 = load i32*, i32** %inout, align 8
  %load_inout = load i32, i32* %deref1, align 4
  %tmpVar = add i32 %load_inout, 1
  store i32 %tmpVar, i32* %deref, align 4
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %baz = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  store i32 7, i32* %baz, align 4
  br label %input

input:                                            ; preds = %entry
  store i32* %baz, i32** getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0), align 8
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
