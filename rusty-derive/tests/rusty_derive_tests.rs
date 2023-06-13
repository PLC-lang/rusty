#[cfg(test)]
mod tests {
    use rusty_derive::Validators;
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
}
