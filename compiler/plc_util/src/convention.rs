//! This module hosts several function related to naming conventions.

use std::fmt::Display;

/// Returns a qualified name in the form of `<qualifier>.<name>`.
pub fn qualified_name<T: AsRef<str> + Display>(qualifier: T, name: T) -> String {
    format!("{qualifier}.{name}")
}

/// Returns a name for internally created types in the form of `__<prefix><original_type_name>`.
pub fn internal_type_name<T: AsRef<str> + Display>(prefix: T, original_type_name: T) -> String {
    format!("__{prefix}{original_type_name}")
}

pub fn generic_binding_type_name<T: AsRef<str> + Display>(pou: T, binding: T) -> String {
    format!("__{pou}__{binding}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn qualified_name() {
        assert_eq!(super::qualified_name("main", "foo"), "main.foo".to_string());
    }

    #[test]
    fn internal_type_name() {
        assert_eq!(super::internal_type_name("POINTER_TO_", "foo"), "__POINTER_TO_foo");
    }
}
