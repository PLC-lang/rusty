
use inkwell::debug_info::{DebugInfoBuilder,  DICompileUnit, DIType, DIScope, DISubprogram, AsDIScope};
use inkwell::context::Context;
use inkwell::builder::Builder;

use crate::ast::*;
use crate::index::DataTypeInformation;

pub struct DebugManager<'ctx> {

    context: Option<&'ctx Context>,
    di_builder: Option<DebugInfoBuilder<'ctx>>,
    compilation_unit: Option<DICompileUnit<'ctx>>,
    scope:  Option<DIScope<'ctx>>,
}

impl <'ctx> DebugManager<'ctx> {
    pub fn create_inactive() -> DebugManager<'ctx> {
        DebugManager::new(None,None, None)
    }

    pub fn activate(&mut self, context: &'ctx Context, di_builder : DebugInfoBuilder<'ctx>, compilation_unit: DICompileUnit<'ctx>) {
        self.context = Some(context);
        self.di_builder = Some(di_builder);
        self.compilation_unit = Some(compilation_unit);
    }
    
    pub fn new(context: Option<&'ctx Context>,  di_builder : Option<DebugInfoBuilder<'ctx>>, compilation_unit: Option<DICompileUnit<'ctx>>) -> DebugManager<'ctx> {
        DebugManager {
            context,
            di_builder,
            compilation_unit,
            scope: None,
        }
    }

    pub fn finalize(&self) {
        if let Some(di_builder) =  &self.di_builder {
            di_builder.finalize();       
        }
    }

    pub fn create_subprogram(&mut self, members : &[(&str, DataTypeInformation<'ctx>)], function : &POU, new_lines : &[usize]) -> Option<(DIType<'ctx>, DISubprogram<'ctx>)>{
        //Exit if dummy
        if self.di_builder.is_none() {
            return None;
        }

        let di_builder = self.di_builder.as_ref().unwrap();
        let compilation_unit = self.compilation_unit.as_ref().unwrap();

        let line_number = get_line_of(new_lines, &function.location.start) as u32 +1 ; //TODO: Line numbers are off by one
        //
        //Create and get a struct type
        let struct_type = self.get_struct_type(function.name.as_str(), line_number, members).unwrap();

        //Add function debug information
        let scope = self.get_current_scope();
        let subroutine_type = di_builder.create_subroutine_type(compilation_unit.get_file(), 
        None,  &[struct_type] , inkwell::debug_info::DIFlagsConstants::PUBLIC, );
        let result = di_builder.create_function(
            scope, 
            function.name.as_str(), 
            None, 
            compilation_unit.get_file(), 
            line_number, 
            subroutine_type, 
            true, 
            true, 
            line_number, 
            inkwell::debug_info::DIFlagsConstants::PUBLIC, 
            false
        );
        self.scope = Some(result.as_debug_info_scope());
        Some((struct_type, result))
    }

    fn get_current_scope(&self) -> DIScope<'ctx> {
        self.scope.or(Some(self.compilation_unit.as_ref().unwrap().get_file().as_debug_info_scope())).unwrap()
    }

    pub fn get_basic_type(&self, name : &str, datatype: &DataTypeInformation) -> Option<DIType<'ctx>> {
         //Exit if dummy
         if self.di_builder.is_none() {
            return None;
        }

        let di_builder = self.di_builder.as_ref().unwrap();
        //TODO : Cache
        match *datatype {
            DataTypeInformation::Integer {size, ..} => {
                di_builder.create_basic_type(
                    name, 
                    size as u64, 
                    0x00, 
                    inkwell::debug_info::DIFlagsConstants::PUBLIC
                ).map(|it| it.as_type()).ok()
            },
            _ => None,
        }   
    }
/*
    pub fn get_instance_variable(&self, name : &str, debug_type : DIType<'ctx>, line : u32,global_value : PointerValue,block : BasicBlock, debug_location : DILocation) {
        //Exit if dummy
        if self.di_builder.is_none() {
            return;
        }

        let context = self.context.unwrap();
        let di_builder = self.di_builder.as_ref().unwrap();
        let compilation_unit = self.compilation_unit.as_ref().unwrap();
        
        let instance_variable = Some(di_builder.create_parameter_variable(
            self.get_current_scope(), 
            name, 
            0, 
            compilation_unit.get_file(), 
            line, 
            debug_type, 
            false, 
            inkwell::debug_info::DIFlagsConstants::PUBLIC,
        ));

        let debug_loc = di_builder.create_debug_location(context, line, 0, self.get_current_scope(), None);
        di_builder.insert_declare_at_end(
            global_value,
             instance_variable, 
             Some(di_builder.create_expression(Vec::new())), 
             debug_loc, 
             block
        );
    }
*/
    /* fn get_variables_debug_information(
        &self,
        debugger: &mut DebugManager<'ctx>,
        variables: &Vec<Variable>,
    ) -> Vec<(String, Option<DIType<'ctx>>)>
    {
        let mut types: Vec<(String, Option<DIType<'ctx>>)> = Vec::new();
        for variable in variables {
            let (_, debug_type)  = self.get_debug_type(debugger, &variable.data_type).unwrap();
            types.push((variable.name.clone(), debug_type));
        }
        types
    }

    fn get_debug_type(&self, name: &str, data_type: &DataTypeInformation<'ctx>) -> Option<(BasicTypeEnum<'ctx>, Option<DIType<'ctx>>)> {
        Some(data_type.get_type(), self.get_basic_type(name, data_type))
    }
*/
    pub fn get_struct_type(&self, name : &str, line : u32, members : &[(&str, DataTypeInformation<'ctx>)]) -> Option<DIType<'ctx>> {
        //Exit if dummy
        if self.di_builder.is_none() {
            return None;
        }

        let di_builder = self.di_builder.as_ref().unwrap();
        let compilation_unit = self.compilation_unit.as_ref().unwrap();


        let mut elements = Vec::new();
        let mut size = 0;
        for (name, datatype) in members {
            size = size + datatype.get_size();
            if let Some(member) = self.get_basic_type(name, datatype) {
                elements.push(member);
            }
        }

        let result = di_builder.create_struct_type(
            self.get_current_scope(), 
            name, 
            compilation_unit.get_file(), 
            line, 
            size as u64, 
            8, 
            inkwell::debug_info::DIFlagsConstants::PUBLIC, 
            None, 
            elements.as_slice(), 
            0, 
            None, 
            format!("{}_debug_interface", name.to_string()).as_str(),
        );

        di_builder.create_typedef(result.as_type(), name, compilation_unit.get_file(), line
            , self.get_current_scope(), 8);


        let mut current_offset = 0;
        for (i, (name, datatype)) in members.iter().enumerate() {
            di_builder.create_member_type(result.as_debug_info_scope(), name, compilation_unit.get_file(), line, (datatype.get_size() * 8) as u64, 8, current_offset, 
                inkwell::debug_info::DIFlagsConstants::PUBLIC, 
                elements[i],
            );
            current_offset = current_offset + (datatype.get_size() * 8) as u64; //TODO : Add align magic
        }

        Some(result.as_type())
    }

    pub fn append_breakpoint_location(&mut self, builder : &Builder, line : u32) {
        //Exit if dummy
        if self.di_builder.is_none() {
            return;
        }
        let context = self.context.unwrap();
        let di_builder = self.di_builder.as_ref().unwrap();
        
        let parent_scope = self.get_current_scope();
        //let compilation_unit = self.compilation_unit.as_ref().unwrap();
        //let lexical_block = di_builder.create_lexical_block(parent_scope, compilation_unit.get_file(), line, 0);
        
        let result = di_builder.create_debug_location(context, line, 0, parent_scope, None);
        builder.set_current_debug_location(context, result)

    }

    pub fn remove_scope(&mut self) {
        self.scope = None;
    }
}
