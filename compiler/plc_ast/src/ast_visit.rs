use crate::ast::{DataType, DataTypeDeclaration, Pou, VariableBlock};

trait AstVisitorMut: Sized {
    fn visit_pou(&mut self, pou: &Pou) {
        default_visit_pou(self, pou);
    }

    fn visit_variable_block(&mut self, block: &VariableBlock) {}

    fn visit_pou_return_type(&mut self, decl: &DataTypeDeclaration) {}
}

pub fn default_visit_pou<T: AstVisitorMut>(vis: &mut T, pou: &Pou) {
    pou.variable_blocks.iter().for_each(|b| vis.visit_variable_block(b));
    pou.return_type.as_ref().inspect(|rt| vis.visit_pou_return_type(rt));
}

pub fn default_visit_variable_block<T: AstVisitorMut>(vis: &mut T, block: &VariableBlock) {

}

pub fn default_visit_pou_return_type<T: AstVisitorMut>(vis: &mut T, decl: &DataTypeDeclaration) {
    
}
