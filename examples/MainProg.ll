; ModuleID = 'main'
source_filename = "main"

%MainProg_interface = type { i16 }

@MainProg_instance = common global %MainProg_interface zeroinitializer

define void @MainProg(%MainProg_interface* %0) {
entry:
  %counter = getelementptr inbounds %MainProg_interface, %MainProg_interface* %0, i32 0, i32 0
  %load_counter = load i16, i16* %counter
  %1 = sext i16 %load_counter to i32
  %tmpVar = add i32 %1, 1
  %2 = trunc i32 %tmpVar to i16
  store i16 %2, i16* %counter
  call void @MainProg(%MainProg_interface* @MainProg_instance)
  %load_counter1 = load i16, i16* %counter
  store i16 %load_counter1, %MainProg_interface* @MainProg_instance
  ret void
}
