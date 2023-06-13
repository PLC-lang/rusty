#[cfg(test)]
mod tests {
    use rusty_derive::{GetAstId, Validators};
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Diagnostic {
        Error,
        OtherError,
        Warning,
    }
    pub trait Validators {
        fn push_diagnostic(&mut self, diagnostic: Diagnostic);

        fn take_diagnostics(&mut self) -> Vec<Diagnostic>;
    }

    #[derive(Default, Validators)]
    pub struct MockValidator {
        pub diagnostics: Vec<Diagnostic>,
    }

    #[test]
    fn derive_validators_implements_trait_functions_correctly() {
        let mut validator = MockValidator::default();

        validator.push_diagnostic(Diagnostic::Error);
        validator.push_diagnostic(Diagnostic::OtherError);
        validator.push_diagnostic(Diagnostic::Warning);

        let expected = vec![Diagnostic::Error, Diagnostic::OtherError, Diagnostic::Warning];
        assert_eq!(expected, validator.diagnostics);

        let mut all_diagnostics = vec![];
        all_diagnostics.append(&mut validator.take_diagnostics());

        assert_eq!(expected, all_diagnostics);
    }

    type AstId = usize;
    #[derive(GetAstId)]
    #[allow(dead_code)]
    enum AstStatement {
        A { id: usize, truth: bool },
        B { id: usize, words: String },
        C { id: usize, mirror: Box<AstStatement> },
    }
    #[test]
    fn foo() {
        let (a, b, c) = (
            AstStatement::A { id: 27, truth: true },
            AstStatement::B { id: 200, words: String::from("hello") },
            AstStatement::C { id: 111, mirror: Box::new(AstStatement::A { id: 12, truth: false }) },
        );

        let expected = (27, 200, 111);
        let result = (a.get_id(), b.get_id(), c.get_id());
        assert_eq!(expected, result);
    }
}
