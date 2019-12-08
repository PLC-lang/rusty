; ModuleID = 'main'
source_filename = "main"

%refs_interface = type { i32, i32 }

@refs_instance = common global %refs_interface zeroinitializer

define i32 @main() {
entry:
  %deref = load i32, i32* getelementptr inbounds (%refs_interface, %refs_interface* @refs_instance, i32 0, i32 0)
  %tmpVar = add i32 %deref, 7
  ret i32 %tmpVar
}