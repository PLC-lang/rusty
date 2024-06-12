use plc_ast::{
    ast::LinkageType,
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocationFactory;

use crate::{lexer, parser};

/// This is a simple visitor that collects all identifiers and literals in a given body
/// It is used by unit tests, to easily see if all identifiers (even the deeply nested ones) were visited
/// and therefore that all subtrees of the AST were visited.
///
/// e.g. if we have a source code like this:
/// ```
/// PROGRAM prg
///    foo (a := b, c => 4);
/// END_PROGRAM
/// ```
/// The visitor should collect the following identifiers/literals: "foo", "a", "b", "c" and "4"
#[derive(Default)]
struct IdentifierCollector {
    identifiers: Vec<String>,
}

impl AstVisitor for IdentifierCollector {
    fn visit_identifier(&mut self, stmt: &str, _node: &plc_ast::ast::AstNode) {
        self.identifiers.push(stmt.to_string());
    }

    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, _node: &plc_ast::ast::AstNode) {
        self.identifiers.push(stmt.get_literal_value());
    }
}

/// Helper function to create a vector of strings with all characters in the range from start to end
fn get_character_range(start: char, end: char) -> Vec<String> {
    (start as u8..=end as u8).map(|c| c as char).map(|c| c.to_string()).collect()
}

/// Helper function to collect all identifiers in a given source code
/// using the IdentifierCollector visitor
fn collect_identifiers(src: &str) -> IdentifierCollector {
    let mut visitor = IdentifierCollector::default();
    visit(src, &mut visitor);
    visitor.identifiers.sort();
    visitor
}

/// Helper function to visit a given source code with a given visitor
fn visit(src: &str, visitor: &mut impl AstVisitor) {
    let id_provider = IdProvider::default();
    let (compilation_unit, _) = parser::parse(
        lexer::lex_with_ids(src, id_provider.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    visitor.visit_compilation_unit(&compilation_unit)
}

#[test]
fn test_visit_arithmetic_expressions() {
    // GIVEN a source code with arithmetic expressions
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a;
            b;
            c := NOT d;
            e := f MOD g;
            h := (i / j / k - (l + m)) * n;
        END_PROGRAM",
    );
    // THEN we expect to also visit subexpressions in binary and unary expressions
    assert_eq!(get_character_range('a', 'n'), visitor.identifiers);
}

#[test]
fn test_visit_expression_list() {
    // GIVEN a source code with an expression list
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a,b,c;
        END_PROGRAM",
    );
    // THEN we expect to visit all identifiers in the expression list
    assert_eq!(get_character_range('a', 'c'), visitor.identifiers);
}

#[test]
fn test_if_statement() {
    // GIVEN a source code with an if statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            IF a THEN
                b := c;
            ELSIF d THEN
                e := f;
            ELSE
                g := h;
            END_IF;
        END_PROGRAM",
    );
    // THEN we expect to visit the condition, the body, the elseif condition and body and the else body
    assert_eq!(get_character_range('a', 'h'), visitor.identifiers);
}

#[test]
fn test_visit_for_loop_statement() {
    // GIVEN a source code with a for loop statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            FOR a := b TO c BY d DO
                e;
                f;
            END_FOR;
        END_PROGRAM",
    );
    // THEN we expect to visit the loop variable, the start, end and step expressions and the loop body
    assert_eq!(get_character_range('a', 'f'), visitor.identifiers);
}

#[test]
fn test_visit_while_loop_statement() {
    // GIVEN a source code with a while loop statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            WHILE a < b DO
                c;
                d;
            END_WHILE;
        END_PROGRAM",
    );
    // THEN we expect to visit the condition and the loop body
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}
#[test]
fn test_visit_repeat_loop_statement() {
    // GIVEN a source code with a repeat loop statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            REPEAT
                a;
                b;
            UNTIL c > d;
        END_PROGRAM",
    );
    // THEN we expect to visit the loop body and the condition
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_case_statement() {
    // GIVEN a source code with a case statement

    let visitor = collect_identifiers(
        "
        PROGRAM prg
            CASE a OF
                b:
                    c;
                    d;
                e, f:
                    g;
                    h;
                ELSE
                    i;
                    j;
            END_CASE;
        END_PROGRAM",
    );
    // THEN we expect to visit the case expression, the case labels, their bodies and the else body
    assert_eq!(get_character_range('a', 'j'), visitor.identifiers);
}

#[test]
fn test_visit_multiplied_statement() {
    // GIVEN a source code with a multiplied statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            3(a+b);
        END_PROGRAM",
    );
    // THEN we expect to visit the multiplied expression and its subexpressions
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visist_array_expressions() {
    // GIVEN a source code with array expressions
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a[b];
            c[d,e+f];
            g[h+i][j+k];
        END_PROGRAM",
    );
    // THEN we expect to visit the array expressions and the array-accessor expressions
    assert_eq!(get_character_range('a', 'k'), visitor.identifiers);
}

#[test]
fn test_visit_range_statement() {
    // GIVEN a source code with range statements
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a..b;
        END_PROGRAM",
    );
    // THEN we expect to visit the start and end expressions of the range
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visit_assignment_expressions() {
    // GIVEN a source code with assignment expressions
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a := b;
            c => d;
            e =>;
        END_PROGRAM",
    );
    // THEN we expect to visit the left and right side of the assignment expressions
    assert_eq!(get_character_range('a', 'e'), visitor.identifiers);
}

#[test]
fn test_visit_direct_access_statement_expressions() {
    // GIVEN a source code with direct access expressions
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            %IW1.2.3; 
            %MD4;
        END_PROGRAM",
    );
    // THEN we expect to visit all segments of the direct access
    assert_eq!(get_character_range('1', '4'), visitor.identifiers);
}

#[test]
fn test_visit_call_statements() {
    // GIVEN a source code with call statements
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a();
            b(c,d);
            e(f:=(g), h=>i);
        END_PROGRAM",
    );
    // THEN we expect to visit the function name and all arguments
    assert_eq!(get_character_range('a', 'i'), visitor.identifiers);
}

#[test]
fn test_visit_return_statement() {
    // GIVEN a source code with a return statement
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        FUNCTION prg : INT
            RETURN a + b;
        END_PROGRAM",
    );
    // THEN we expect to visit the return expression
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visit_into_var_global() {
    // GIVEN a source code with a var_global section
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        VAR_GLOBAL
            a : INT := c;
            c : INT := d;
        END_VAR",
    );
    // THEN we expect to visit all initializers (variable names are no AstStatements!)
    assert_eq!(get_character_range('c', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_data_type_declaration() {
    // GIVEN a visitor that collects variables, enum elements and range expressions
    struct FieldCollector {
        fields: Vec<String>,
    }

    // This is a simple visitor that collects all field names in a datatype
    impl AstVisitor for FieldCollector {
        fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
            self.fields.push(variable.name.clone());
            variable.walk(self);
        }

        fn visit_enum_element(&mut self, element: &plc_ast::ast::AstNode) {
            if let Some(name) = element.get_flat_reference_name() {
                self.fields.push(name.to_string());
            }
            element.walk(self);
        }

        fn visit_range_statement(
            &mut self,
            stmt: &plc_ast::ast::RangeStatement,
            _node: &plc_ast::ast::AstNode,
        ) {
            if let Some((start, end)) =
                stmt.start.get_flat_reference_name().zip(stmt.end.get_flat_reference_name())
            {
                self.fields.push(start.to_string());
                self.fields.push(end.to_string());
            }
            stmt.walk(self);
        }
    }
    let mut visitor = FieldCollector { fields: vec![] };
    // WHEN we visit a source code with a complex datatype
    visit(
        "
        TYPE myStruct: STRUCT
            a, b, c: DINT;
            s: STRING;
            e: (enum1, enum2, enum3);
        END_STRUCT;
        END_TYPE

        TYPE MyEnum: (myEnum1, myEnum2, myEnum3);
        END_TYPE

        TYPE MySubRange: INT(max..min); END_TYPE

        TYPE MyArray: ARRAY[start..end] OF INT; END_TYPE
      ",
        &mut visitor,
    );

    visitor.fields.sort();
    // THEN we expect to visit all fields, enum elements and range expressions
    assert_eq!(
        vec![
            "a", "b", "c", "e", "end", "enum1", "enum2", "enum3", "max", "min", "myEnum1", "myEnum2",
            "myEnum3", "s", "start"
        ],
        visitor.fields
    );
}

#[test]
fn test_count_assignments() {
    // GIVEN a visitor that counts assignments
    struct AssignmentCounter {
        count: usize,
    }

    impl AstVisitor for AssignmentCounter {
        fn visit_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
            self.count += 1;
            stmt.walk(self)
        }

        fn visit_output_assignment(
            &mut self,
            stmt: &plc_ast::ast::Assignment,
            _node: &plc_ast::ast::AstNode,
        ) {
            self.count += 1;
            stmt.walk(self)
        }
    }

    let id_provider = IdProvider::default();
    let (compilation_unit, _) = parser::parse(
        lexer::lex_with_ids(
            "
            PROGRAM prg
                a := b;
                c => d;
                e := f;
                foo(a := baz(x := 2, z => 3));
            END_PROGRAM",
            id_provider.clone(),
            SourceLocationFactory::internal(""),
        ),
        LinkageType::Internal,
        "test.st",
    );

    let mut visitor = AssignmentCounter { count: 0 };
    // WHEN we visit a source code with assignments
    for st in &compilation_unit.implementations[0].statements {
        visitor.visit(st);
    }
    // THEN we expect to visit all assignments
    assert_eq!(6, visitor.count);
}

#[test]
fn test_visit_datatype_initializers_statement() {
    // GIVEN a source code with datatype initializers
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        TYPE MyStruct: STRUCT
            field1: DINT := a;
            field2: DINT := (b + c);
            field3: ARRAY[1..3] OF DINT := 4(d);
            field4: ARRAY[4..7] OF DINT := (e, f, g, h);
            field5: (i := j, k := l) := m;

        END_STRUCT
        END_TYPE",
    );
    // THEN we expect to visit all initializers and enum elements
    let mut expected = ["1", "3", "4", "7"].iter().map(|c| c.to_string()).collect::<Vec<String>>();

    expected.extend(get_character_range('a', 'm'));
    assert_eq!(expected, visitor.identifiers);
}

#[test]
fn test_visit_array_declaration_statement() {
    // GIVEN a source code with array declarations
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        TYPE MyArray: ARRAY[(a+b)..(c+d)] OF INT; END_TYPE",
    );
    // THEN we expect to visit the start and end expressions of the array
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_qualified_expressions() {
    // GIVEN a source code with qualified expressions
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a.b;
            c.d^.e;
            f.g[h].i;
        END_PROGRAM",
    );
    // THEN we expect to visit all segments in the qualified expressions
    assert_eq!(get_character_range('a', 'i'), visitor.identifiers);
}

#[test]
fn test_visit_variable_block() {
    // GIVEN a source code with a variable block
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            VAR_INPUT
                a : INT := X;
            END_VAR
            VAR_OUTPUT
                b : INT := Y;
            END_VAR
            VAR CONSTANT
                c : INT
            END_VAR
        END_PROGRAM",
    );
    // THEN we expect to visit all variables and their initializers
    assert_eq!(get_character_range('X', 'Y'), visitor.identifiers);
}

#[test]
fn test_visit_continue_exit() {
    // THIS test is mainly here to cover the default visit implementation of Continue, Exit and EmptyStatement
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            CONTINUE;
            EXIT;
            ;
        END_PROGRAM",
    );
    assert_eq!(0, visitor.identifiers.len());
}

#[test]
fn test_visit_default_value() {
    // GIVEN a Visitor that visits default values
    struct DefaultValueCollector {
        visited: bool,
    }

    // This is a simple visitor that collects all field names in a datatype
    impl AstVisitor for DefaultValueCollector {
        fn visit_default_value(&mut self, _stmt: &plc_ast::ast::DefaultValue, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visitor = DefaultValueCollector { visited: false };
    // WHEN we visit a source code with a default value
    visit(
        "
        VAR_GLOBAL CONSTANT
            a : INT;
        END_VAR
        ",
        &mut visitor,
    );
    // THEN we expect to visit the default value
    assert!(visitor.visited);
}

#[test]
fn test_visit_direct_access() {
    // GIVEN a Visitor that visits direct accesses
    struct Visited {
        visited: bool,
    }

    impl AstVisitor for Visited {
        fn visit_direct_access(&mut self, _stmt: &plc_ast::ast::DirectAccess, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visitor = Visited { visited: false };
    // WHEN we visit a source code with a direct access
    visit(
        "
        PROGRAM prg
            x.1;
        ",
        &mut visitor,
    );
    // THEN we expect to visit the direct access
    assert!(visitor.visited);

    let v = collect_identifiers(
        "
        PROGRAM prg
            x.1;
        ",
    );
    assert_eq!(vec!["1", "x"], v.identifiers);
}

#[test]
fn test_invalid_case_condition() {
    // this tests ensures that we visit "invalid" statements. (see parser's behavior in parse_statement)
    struct Visited {
        visited: bool,
    }

    impl AstVisitor for Visited {
        fn visit_case_condition(&mut self, _child: &plc_ast::ast::AstNode, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visitor = Visited { visited: false };

    visit(
        "
        PROGRAM prg
            x:
        ",
        &mut visitor,
    );
    assert!(visitor.visited);
}

#[test]
fn test_visit_string_declaration() {
    // GIVEN a source code with a string declaration
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            VAR
                str: STRING(X);
            END_VAR
        ",
    );
    // THEN we expect to visit the string length
    assert_eq!(vec!["X"], visitor.identifiers);
}

#[test]
fn test_visit_pointer_declaration() {
    // GIVEN a source code with a pointer declaration
    // WHEN we visit all nodes in the AST
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            VAR
                str: POINTER TO ARRAY[a..b] OF INT := c;
            END_VAR
        ",
    );
    // THEN we expect to visit the pointer type and the initializer
    assert_eq!(get_character_range('a', 'c'), visitor.identifiers);
}
