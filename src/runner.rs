use inkwell::{
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
};

use crate::{compile_module, diagnostics::Diagnostician, SourceCode, SourceContainer};

type MainFunction<T, U> = unsafe extern "C" fn(*mut T) -> U;
type MainEmptyFunction<U> = unsafe extern "C" fn() -> U;

#[allow(dead_code)]
#[repr(C)]
pub struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

pub trait Compilable {
    type T: SourceContainer;
    fn containers(self) -> Vec<Self::T>;
}

impl Compilable for &str {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = Self::T::from(self);
        vec![code]
    }
}

impl Compilable for String {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = self.into();
        vec![code]
    }
}

impl<S: SourceContainer> Compilable for Vec<S> {
    type T = S;
    fn containers(self) -> Vec<Self::T> {
        self
    }
}

impl Compilable for SourceCode {
    type T = Self;

    fn containers(self) -> Vec<Self::T> {
        vec![self]
    }
}

///
/// Runs the function given by `name` inside the compiled execution engine code.
/// Returns the value returned by calling the function
///
pub fn run<T, U>(exec_engine: &ExecutionEngine, name: &str, params: &mut T) -> U {
    unsafe {
        let main: JitFunction<MainFunction<T, U>> = exec_engine.get_function(name).unwrap();
        let main_t_ptr = &mut *params as *mut _;

        main.call(main_t_ptr)
    }
}

///
/// Runs the function given by `name` inside the compiled execution engine code.
/// Returns the value returned by calling the function
///
pub fn run_no_param<U>(exec_engine: &ExecutionEngine, name: &str) -> U {
    unsafe {
        let main: JitFunction<MainEmptyFunction<U>> = exec_engine.get_function(name).unwrap();
        main.call()
    }
}

///
/// Compiles and runs the given sources
/// Sources must be `Compilable`, default implementations include `String` and `&str`
/// An implementation is also provided for `Vec<SourceContainer>`
///
pub fn compile<T: Compilable>(context: &Context, source: T) -> ExecutionEngine {
    let source = source.containers();
    let (_, code_gen) = compile_module(
        context,
        source,
        vec![],
        None,
        Diagnostician::null_diagnostician(),
    )
    .unwrap();
    code_gen.module.print_to_stderr();
    code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap()
}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: Context = Context::create();
    let exec_engine = compile(&context, source);
    run::<T, U>(&exec_engine, "main", params)
}
