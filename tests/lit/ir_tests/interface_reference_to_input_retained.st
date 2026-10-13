// RUN: %COMPILE_IR %s | %CHECK_IR %s
// A function block keeps a REFERENCE TO interface input between calls, so the fat pointer that wraps a
// concrete instance is a global of the unit rather than an allocation on the binding function's stack.
// The global is named after the binding function, so objects compiled separately never clash.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Consumer
    VAR_INPUT
        in1 : REFERENCE TO IA;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    globalConsumer : Consumer;
    globalInstance : FbA;
END_VAR

// The IR dump of a multi-unit build renames struct types, so only the global's name is checked.
// CHECK: @__fatpointer_4bind_0 = global %{{[A-Za-z_0-9]+}} zeroinitializer

FUNCTION bind
    globalConsumer(globalInstance);
END_FUNCTION

// CHECK-LABEL: define void @bind()
// CHECK-NOT: alloca
// CHECK: store ptr @globalInstance, ptr @__fatpointer_4bind_0
// CHECK: store ptr @__itable_IA_FbA_instance, ptr getelementptr inbounds {{.*}}ptr @__fatpointer_4bind_0
// CHECK: store ptr @__fatpointer_4bind_0, ptr getelementptr inbounds {{.*}}ptr @globalConsumer
// CHECK: ret void
