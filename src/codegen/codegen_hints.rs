use inkwell::types::BasicTypeEnum;


pub struct CodeGenHints<'a> {
    type_stack: Vec<BasicTypeEnum<'a>>,
}

impl<'a> CodeGenHints<'a> {

    pub fn new() -> CodeGenHints<'a> {
        CodeGenHints{
            type_stack: Vec::new(),
        }
    }

    pub fn push_type_hint(&mut self, hint: BasicTypeEnum<'a>) {
        self.type_stack.push(hint);
    }

    pub fn pop_type_hint(&mut self) -> Option<BasicTypeEnum<'a>> {
        self.type_stack.pop()
    }

    pub fn get_current_type(&self) -> Option<&BasicTypeEnum<'a>> {
        self.type_stack.last()
    }
}


