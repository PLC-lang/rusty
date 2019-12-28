use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}
//impl<'ctx> CodeGen<'ctx> {
pub fn generate() {
    let context = Context::create();
    let module = context.create_module("main");
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
    };
    let intType = context.i32_type();
    let one_1 = intType.const_int(1, true);
    let one_2 = intType.const_int(1, true);

    let function =
        codegen
            .module
            .add_function("main", context.i32_type().fn_type(&[], false), None);

    let basic_block = codegen.context.append_basic_block(function, "entry");

    codegen.builder.position_at_end(&basic_block);
    let sum = codegen.builder.build_int_add(one_1, one_2, "sum");

    codegen.builder.build_return(Some(&sum));
    codegen.module.print_to_stderr();
}

//}
