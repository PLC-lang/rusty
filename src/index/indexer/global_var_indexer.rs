use plc_ast::ast::LinkageType;

use crate::index::{HardwareBinding, Index, VariableIndexEntry};

pub struct VarGlobalIndexer<'i> {
    constant: bool,
    linkage: LinkageType,
    index: &'i mut Index,
}

impl VarGlobalIndexer<'_> {
    pub fn new(constant: bool, linkage: LinkageType, index: &mut Index) -> VarGlobalIndexer<'_> {
        VarGlobalIndexer { constant, linkage, index }
    }

    pub fn visit_variable(&mut self, var: &plc_ast::ast::Variable) {
        let target_type = var.data_type_declaration.get_name().unwrap_or_default();
        let initializer = self.index.get_mut_const_expressions().maybe_add_constant_expression(
            var.initializer.clone(),
            target_type,
            None,
        );
        let variable = VariableIndexEntry::create_global(
            &var.name,
            &var.name,
            var.data_type_declaration.get_name().expect("named variable datatype"),
            var.location.clone(),
        )
        .set_initial_value(initializer)
        .set_constant(self.constant)
        .set_linkage(self.linkage)
        .set_hardware_binding(
            var.address.as_ref().and_then(|it| HardwareBinding::from_statement(self.index, it, None)),
        );
        self.index.register_global_variable(&var.name, variable);
    }
}
