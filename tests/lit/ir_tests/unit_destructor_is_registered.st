// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A unit whose globals or programs need cleanup gets a destructor that is registered in
// `llvm.global_dtors`. Types get a destructor only when they declare `FB_EXIT` or own an
// instance of a type that does.

FUNCTION_BLOCK Connection
    METHOD FB_EXIT
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Client
    VAR
        connection: Connection;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Plain
    VAR
        value: DINT;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    client: Client;
    plain: Plain;
END_VAR

// `llvm.global_dtors` is the list of functions that run when the module is unloaded. It has one
// entry, the destructor of this unit.
// CHECK: @llvm.global_dtors = appending global [1 x {{.*}}ptr @__unit_{{.*}}__dtor{{.*}}
// CHECK-NOT: Plain__dtor
// CHECK: define void @__unit_{{.*}}__dtor()
// CHECK-NEXT: entry:
// CHECK-NEXT: call void @Client__dtor(ptr @client)
// CHECK-NOT: Plain__dtor
// CHECK: define void @Client__dtor(ptr %0)
// CHECK: call void @Connection__dtor(ptr %connection)
// CHECK-NOT: Plain__dtor
// CHECK: define void @Connection__dtor(ptr %0)
// CHECK: call void @Connection__FB_EXIT(ptr {{.*}})
// CHECK-NOT: Plain__dtor
