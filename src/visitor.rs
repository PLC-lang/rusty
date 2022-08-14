use std::iter;

use crate::ast::AstStatement;


trait Qualifier {
    fn qualify_name(&self, name : &str) -> String; //:(
}

struct Scope {
    qualifiers: Vec<Box<dyn Qualifier>>
}

struct GlobalScope{}
impl Qualifier for GlobalScope {
    fn qualify_name(&self, name : &str) -> String {
        name.into()
    }
}

struct LocalScope{
    qualifier: String
}

impl Qualifier for LocalScope{
    fn qualify_name(&self, name : &str) -> String {
        format!("{:}.{:}", self.qualifier, name)
    }
}

trait StatementVisitor<T> {
    fn visit(&self, s: &AstStatement, scope: &Scope, ctx: &T) {
        match s {
            // 1 boxed child
            AstStatement::CastStatement { target: r, .. }
            | AstStatement::PointerAccess { reference: r, .. }
            | AstStatement::DirectAccess { index: r, .. }
            | AstStatement::UnaryExpression { value: r, .. }
            | AstStatement::CaseCondition { condition: r, .. }
            | AstStatement::MultipliedStatement { element: r, .. } => self.do_visit(r, scope, ctx),

            AstStatement::QualifiedReference { elements, id } => todo!(),
            AstStatement::HardwareAccess { address, .. } => todo!(),
            AstStatement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.do_visit(e, scope, ctx)),

            //2 children
            AstStatement::Assignment {
                left: l, right: r, ..
            }
            | AstStatement::RangeStatement {
                start: l, end: r, ..
            }
            | AstStatement::BinaryExpression {
                left: l, right: r, ..
            }
            | AstStatement::ArrayAccess {
                reference: l,
                access: r,
                ..
            }
            | AstStatement::OutputAssignment {
                left: l, right: r, ..
            } => {
                self.do_visit(l, scope, ctx);
                self.do_visit(r, scope, ctx);
            }
            AstStatement::CallStatement {
                operator,
                parameters,
                ..
            } => {
                self.do_visit(operator, scope, ctx);
                if let Some(p) = parameters.as_ref() {
                    self.do_visit(p, scope, ctx);
                }
            }
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks
                    .iter()
                    .flat_map(|it| iter::once(it.condition.as_ref()).chain(it.body.iter()))
                    .chain(else_block.iter())
                    .for_each(|it| self.visit(it, scope, ctx));
            }
            AstStatement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => iter::once(counter.as_ref())
                .chain(iter::once(start.as_ref()))
                .chain(iter::once(end.as_ref()))
                .chain(by_step.as_ref().map(|it| it.as_ref()).into_iter())
                .chain(body.iter())
                .for_each(|it| self.do_visit(it, scope, ctx)),

            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => iter::once(condition.as_ref())
                .chain(body.iter())
                .for_each(|it| self.do_visit(it, scope, ctx)),

            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => iter::once(condition.as_ref())
                .chain(body.iter())
                .for_each(|it| self.do_visit(it, scope, ctx)),
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => iter::once(selector.as_ref())
                .chain(
                    case_blocks
                        .iter()
                        .flat_map(|it| iter::once(it.condition.as_ref()).chain(it.body.iter())),
                )
                .chain(else_block.iter())
                .for_each(|it| self.do_visit(it, scope, ctx)),
            _ => {}
        }
    }

    fn post_handle(&self, s: &AstStatement, scope: &Scope, ctx: &T) {}
    fn pre_handle(&self, s: &AstStatement, scope: &Scope, ctx: &T) -> bool {
        true
    }

    fn do_visit(&self, s: &AstStatement, scope: &Scope, ctx: &T) {
        if self.pre_handle(s, scope, ctx) {
            self.visit(s, scope, ctx);
            self.post_handle(s, scope, ctx);
        }
    }
}
