use crate::literals::{AstLiteral, Date, DateAndTime};

pub trait Acceptor {
    fn accept<'a>(&'a self, visitor: &mut dyn Visitor<ActionType = &'a dyn Acceptor>)
    where
        Self: Sized,
    {
        visitor.visit(self);
    }
}

pub trait Visitor {
    type ActionType;
    fn visit(&mut self, item: Self::ActionType);
}

impl Acceptor for AstLiteral {
    fn accept<'a>(&'a self, visitor: &mut dyn Visitor<ActionType = &'a dyn Acceptor>) {
        match self {
            AstLiteral::Null => {}
            AstLiteral::Integer(val) => val.accept(visitor),
            AstLiteral::Date(date) => {
                date.accept(visitor);
            }
            AstLiteral::DateAndTime(datetime) => {
                datetime.accept(visitor);
            }
            AstLiteral::TimeOfDay(_) => todo!(),
            AstLiteral::Time(_) => todo!(),
            AstLiteral::Real(_) => todo!(),
            AstLiteral::Bool(_) => todo!(),
            AstLiteral::String(_) => todo!(),
            AstLiteral::Array(_) => todo!(),
        }
        visitor.visit(self)
    }
}

impl Acceptor for DateAndTime {}
impl Acceptor for Date {}
impl Acceptor for i128 {}

// impl Visitor for Resolver {
//     type ActionType = Box<dyn Resolve>;
//     fn visit(&mut self, item: Self::ActionType) {
//         let annotations = item.resolve(self);
//         // self.annotate(annotations, item)
//     }
// }
