/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::{BasicTypeEnum, FloatType, IntType, StructType, VoidType}, values::{FunctionValue, VectorValue}};

use crate::ast::NewLines;


pub struct CodeGenContext<'a, 'b> {
    pub new_lines : NewLines,
    current_function: Option<FunctionValue<'b>>,
    type_stack: Vec<BasicTypeEnum<'a>>,
}

impl <'a, 'b> CodeGenContext<'a, 'b> {
    pub fn new(new_lines: NewLines) -> CodeGenContext<'a, 'b> {
        CodeGenContext {
            new_lines,
            current_function: None,
            type_stack: Vec::new(),
        }
    }

    pub fn set_current_function(&mut self, function: FunctionValue<'b>) {
        self.current_function = Some(function);
    }

    pub fn get_current_function(&self) -> Result<FunctionValue<'b>, String> {
        self.current_function
                    .ok_or_else(|| "No current function available".to_string())
    }
 
    
    pub fn push_type_hint(&mut self, hint: BasicTypeEnum<'a>) {
        self.type_stack.push(hint);
    }
    
    pub fn pop_type_hint(&mut self) -> Option<BasicTypeEnum<'a>> {
        self.type_stack.pop()
    }
    
    pub fn get_current_type(&self) -> Option<BasicTypeEnum<'a>> {
        self.type_stack.last().map(|it| it.clone())
    }
    
    /*pub fn generate_struct_type_stub(&self, name: &str) -> StructType {
        self.llvm.opaque_struct_type(name)
    }
    
    pub fn append_basic_block(&self, name: &str) -> Result<BasicBlock, String> {
        let current_function = self.get_current_function()?;
        Ok(self.llvm.append_basic_block(current_function, name))
    }

    pub fn prepend_basic_block(&self, basic_block: BasicBlock<'a>, name: &str) -> BasicBlock {
        self.llvm.prepend_basic_block(basic_block, name)
    }*/
}