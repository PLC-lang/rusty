use plc::{
    index::Index,
    resolver::{AnnotationMapImpl, ResolvingStrategy, TypeAnnotator, VisitorContext},
};
use plc_ast::{ast::AstNode, provider::IdProvider};

pub mod inheritance;
pub mod initializer;
pub mod retain;
#[cfg(test)]
mod tests;

// TODO: implement `AnnotationMap` trait or create a new one for lowering
pub(crate) struct LoweringResolver<'rslv> {
    index: &'rslv Index,
    ctx: VisitorContext<'rslv>,
}

impl<'rslv> LoweringResolver<'rslv> {
    pub fn new(index: &'rslv Index, id_provider: IdProvider) -> Self {
        let mut ctx = VisitorContext::default();
        ctx.id_provider = id_provider;
        let ctx = ctx.with_resolving_strategy(ResolvingStrategy::default_scopes());
        Self { index, ctx }
    }

    pub fn with_pou(mut self, pou: &'rslv str) -> Self {
        self.ctx = self.ctx.with_pou(pou);
        self
    }

    /// Resolves and annotates the given statement and returns the annotation map.
    pub fn resolve_statement(&self, stmt: &AstNode) -> AnnotationMapImpl {
        let mut annotator = TypeAnnotator::new(self.index);
        annotator.visit_statement(&self.ctx, stmt);
        std::mem::take(&mut annotator.annotation_map)
    }
}
