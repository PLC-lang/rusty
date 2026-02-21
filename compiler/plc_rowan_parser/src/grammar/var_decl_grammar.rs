use crate::SyntaxKind::{self, *};
use crate::T;
use crate::grammar::{name, name_ref, statement_grammar::expression_stmt};
use crate::parser::Parser;

// var_declaration =
//     identifier_list [ location ] ':' type_specification [ ':=' expression ] ';'

pub fn var_declaration(p: &mut Parser) {
    let m = p.start();

    // Parse identifier_list (one or more identifiers separated by commas)
    identifier_list(p);

    // Optional location (AT %...)
    if p.at(T![AT]) {
        location(p);
    }

    // Expect colon
    p.expect(T![:]);

    // Parse type_specification (for now, just use name_ref)
    name_ref(p); //for now type reference
    // type_specification(p);

    // Optional initializer — emit as ExpressionStmt.
    // expression_stmt() will consume the trailing ';' as part of the ExpressionStmt node.
    // When there's no initializer, we consume ';' here directly.
    if p.eat(T![:=]) {
        expression_stmt(p, false);
    }
    p.expect(T![;]);

    m.complete(p, VAR_DECLARATION);
}

// identifier_list = identifier { ',' identifier }
fn identifier_list(p: &mut Parser) {
    let m = p.start();

    // First identifier
    name(p);

    // Additional identifiers
    while p.eat(T![,]) {
        name(p);
    }

    m.complete(p, IDENTIFIER_LIST);
}

// location = 'AT' direct_variable
fn location(p: &mut Parser) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use crate::{
        grammar::tests::{format_tree, parse_with},
        lexed_str::LexedStr,
    };

    use super::*;

    #[test]
    fn test_var_declaration_simple() {
        let input = "x : INT;";
        let input = LexedStr::new(input);
        let output = parse_with(&input, var_declaration);
        insta::assert_snapshot!(format_tree(&output, &input));
    }

    #[test]
    fn test_var_declaration_with_initializer() {
        let input = "x : INT := 42;";
        let input = LexedStr::new(input);
        let output = parse_with(&input, var_declaration);
        insta::assert_snapshot!(format_tree(&output, &input));
    }

    #[test]
    fn test_var_declaration_multiple_vars() {
        let input = "x, y, z : INT;";
        let input = LexedStr::new(input);
        let output = parse_with(&input, var_declaration);
        insta::assert_snapshot!(format_tree(&output, &input));
    }

    #[test]
    fn test_var_declaration_with_location() {
        let input = "x AT %MW0 : INT;";
        let input = LexedStr::new(input);
        let output = parse_with(&input, var_declaration);
        insta::assert_snapshot!(format_tree(&output, &input));
    }

    #[test]
    fn test_var_declaration_error_missing_semicolon() {
        let input = "x : INT";
        let input = LexedStr::new(input);
        let output = parse_with(&input, var_declaration);
        insta::assert_snapshot!(format_tree(&output, &input));
    }
}
