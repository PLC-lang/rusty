use plc_ast::ast::{
    ArgumentProperty, DataTypeDeclaration, PouType, TypeNature, VariableBlock, VariableBlockType,
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

use crate::{
    index::{
        ArgumentType, HardwareBinding, Index, MemberInfo, PouIndexEntry, VariableIndexEntry, VariableType,
    },
    typesystem::{self, DataTypeInformation, StructSource, VarArgs, VOID_TYPE},
};

/// indexer for a single POU
pub struct PouIndexer<'i> {
    index: &'i mut Index,
}

impl<'i> PouIndexer<'i> {
    pub fn new(index: &'i mut Index) -> Self {
        Self { index }
    }

    /// Visits a pou and registers all member variables in the index
    /// Also registers the pou's struct type in the index
    pub fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        //register the pou's member variables
        let (count, mut members, member_varargs) = self.index_pou_variables(pou);

        //register a function's return type as a member variable
        let return_type_name = pou.return_type.as_ref().and_then(|it| it.get_name()).unwrap_or(VOID_TYPE);
        if pou.return_type.is_some() {
            let entry = self.index.create_member_variable(
                MemberInfo {
                    container_name: &pou.name,
                    variable_name: pou.get_return_name(),
                    variable_linkage: ArgumentType::ByVal(VariableType::Return),
                    variable_type_name: return_type_name,
                    is_constant: false,     //return variables are not constants
                    is_var_external: false, // see above
                    binding: None,
                    varargs: None,
                },
                None,
                pou.name_location.clone(),
                count,
            );
            members.push(entry);
        }

        // construct the struct-type that holds the POU's `this` state
        let pou_struct_type = typesystem::DataType {
            name: pou.name.to_string(),
            initial_value: None,
            information: DataTypeInformation::Struct {
                name: pou.name.to_string(),
                members,
                source: StructSource::Pou(pou.kind.clone()),
            },
            nature: TypeNature::Any,
            location: pou.name_location.clone(),
        };

        match &pou.kind {
            PouType::Program => {
                self.index_program(pou, pou_struct_type);
            }
            PouType::FunctionBlock => {
                self.index_function_block(pou, pou_struct_type);
            }
            PouType::Class => {
                self.index_class(pou, pou_struct_type);
            }
            PouType::Function | PouType::Init | PouType::ProjectInit => {
                self.index_function(pou, return_type_name, member_varargs, pou_struct_type);
            }
            PouType::Method { parent } => {
                self.index_method(pou, return_type_name, parent, pou_struct_type);
            }
            _ => {}
        };
    }

    /// Registers a program in the index
    fn index_program(&mut self, pou: &plc_ast::ast::Pou, pou_struct_type: typesystem::DataType) {
        self.index.register_program(&pou.name, pou.name_location.clone(), pou.linkage);
        self.index.register_pou_type(pou_struct_type);
    }

    /// Registers a method in the index
    fn index_method(
        &mut self,
        pou: &plc_ast::ast::Pou,
        return_type_name: &str,
        owner_class: &str,
        pou_struct_type: typesystem::DataType,
    ) {
        self.index.register_pou(PouIndexEntry::create_method_entry(
            &pou.name,
            return_type_name,
            owner_class,
            pou.linkage,
            pou.name_location.clone(),
        ));
        self.index.register_pou_type(pou_struct_type);
    }

    /// Registers a function in the index
    fn index_function(
        &mut self,
        pou: &plc_ast::ast::Pou,
        return_type_name: &str,
        member_varargs: Option<VarArgs>,
        pou_struct_type: typesystem::DataType,
    ) {
        self.index.register_pou(PouIndexEntry::create_function_entry(
            &pou.name,
            return_type_name,
            &pou.generics,
            pou.linkage,
            member_varargs.is_some(),
            pou.name_location.clone(),
            pou.is_const,
        ));
        self.index.register_pou_type(pou_struct_type);
    }

    /// Registers a class in the index
    fn index_class(&mut self, pou: &plc_ast::ast::Pou, pou_struct_type: typesystem::DataType) {
        let global_struct_name = crate::index::get_initializer_name(&pou.name);
        let variable = VariableIndexEntry::create_global(
            &global_struct_name,
            &global_struct_name,
            &pou.name,
            pou.name_location.clone(),
        )
        .set_constant(true)
        .set_linkage(pou.linkage);
        self.index.register_global_initializer(&global_struct_name, variable);
        self.index.register_pou(PouIndexEntry::create_class_entry(
            &pou.name,
            pou.linkage,
            pou.name_location.clone(),
            pou.super_class.clone(),
        ));
        self.index.register_pou_type(pou_struct_type);
    }

    /// Registers a function block in the index
    fn index_function_block(&mut self, pou: &plc_ast::ast::Pou, pou_struct_type: typesystem::DataType) {
        let global_struct_name = crate::index::get_initializer_name(&pou.name);
        let variable = VariableIndexEntry::create_global(
            &global_struct_name,
            &global_struct_name,
            &pou.name,
            pou.name_location.clone(),
        )
        .set_constant(true)
        .set_linkage(pou.linkage);
        self.index.register_global_initializer(&global_struct_name, variable);
        self.index.register_pou(PouIndexEntry::create_function_block_entry(
            &pou.name,
            pou.linkage,
            pou.name_location.clone(),
            pou.super_class.clone().as_deref(),
        ));
        self.index.register_pou_type(pou_struct_type);
    }

    /// Registers all member variables of a POU in the index
    fn index_pou_variables(
        &mut self,
        pou: &plc_ast::ast::Pou,
    ) -> (u32, Vec<VariableIndexEntry>, Option<VarArgs>) {
        let mut count = 0;
        let mut members = Vec::new();
        let mut member_varargs = None;

        for block in &pou.variable_blocks {
            for var in &block.variables {
                let varargs = if let DataTypeDeclaration::DataTypeDefinition {
                    data_type: plc_ast::ast::DataType::VarArgs { referenced_type, sized },
                    ..
                } = &var.data_type_declaration
                {
                    let name = referenced_type
                        .as_ref()
                        .map(|it| &**it)
                        .and_then(DataTypeDeclaration::get_name)
                        .map(|it| it.to_string());
                    Some(if *sized { VarArgs::Sized(name) } else { VarArgs::Unsized(name) })
                } else {
                    None
                };

                if varargs.is_some() {
                    member_varargs.clone_from(&varargs);
                }

                let var_type_name = var.data_type_declaration.get_name().unwrap_or(VOID_TYPE);
                let block_type = get_declaration_type_for(block, &pou.kind);
                let type_name = if block_type.is_by_ref() {
                    //register a pointer type for argument
                    register_byref_pointer_type_for(self.index, var_type_name)
                } else {
                    var_type_name.to_string()
                };
                let initial_value = self.index.get_mut_const_expressions().maybe_add_constant_expression(
                    var.initializer.clone(),
                    type_name.as_str(),
                    Some(pou.name.clone()),
                    Some(var.get_name().to_string()),
                );

                let binding = var
                    .address
                    .as_ref()
                    .and_then(|it| HardwareBinding::from_statement(self.index, it, Some(pou.name.clone())));

                let entry = self.index.create_member_variable(
                    MemberInfo {
                        container_name: &pou.name,
                        variable_name: &var.name,
                        variable_linkage: block_type,
                        variable_type_name: &type_name,
                        is_constant: block.constant,
                        is_var_external: matches!(block.variable_block_type, VariableBlockType::External),
                        binding,
                        varargs,
                    },
                    initial_value,
                    var.location.clone(),
                    count,
                );
                members.push(entry);
                count += 1;
            }
        }
        (count, members, member_varargs)
    }
}

/// returns the declaration type (ByRef or ByVal) for the given VariableBlock (VAR_INPUT, VAR_OUTPUT, VAR_INOUT, etc.)
fn get_declaration_type_for(block: &VariableBlock, pou_type: &PouType) -> ArgumentType {
    if matches!(
        block.variable_block_type,
        VariableBlockType::InOut | VariableBlockType::Input(ArgumentProperty::ByRef)
    ) {
        ArgumentType::ByRef(get_variable_type_from_block(block))
    } else if block.variable_block_type == VariableBlockType::Output {
        // outputs differ depending on pou type
        match pou_type {
            PouType::Function => ArgumentType::ByRef(get_variable_type_from_block(block)),
            _ => ArgumentType::ByVal(get_variable_type_from_block(block)),
        }
    } else {
        ArgumentType::ByVal(get_variable_type_from_block(block))
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Temp => VariableType::Temp,
        VariableBlockType::Input(_) => VariableType::Input,
        VariableBlockType::Output => VariableType::Output,
        VariableBlockType::Global => VariableType::Global,
        VariableBlockType::InOut => VariableType::InOut,
        VariableBlockType::External => VariableType::External,
        VariableBlockType::Property => VariableType::Property,
    }
}

/// registers an auto-deref pointer type for the inner_type_name if it does not already exist
fn register_byref_pointer_type_for(index: &mut Index, inner_type_name: &str) -> String {
    //get unique name
    let type_name = internal_type_name("auto_pointer_to_", inner_type_name);

    //check if type was already created
    if index.find_effective_type_by_name(type_name.as_str()).is_none() {
        //generate a pointertype for the variable
        index.register_type(typesystem::DataType {
            name: type_name.clone(),
            initial_value: None,
            information: DataTypeInformation::Pointer {
                name: type_name.clone(),
                inner_type_name: inner_type_name.to_string(),
                auto_deref: Some(plc_ast::ast::AutoDerefType::Default),
            },
            nature: TypeNature::Any,
            location: SourceLocation::internal(),
        });
    }

    type_name
}
