use plc_ast::{
    ast::LinkageType,
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocationFactory;

use crate::{lexer, parser};

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

fn get_character_range(start: char, end: char) -> Vec<String> {
    (start as u8..=end as u8).map(|c| c as char).map(|c| c.to_string()).collect()
}

fn collect_identifiers(src: &str) -> IdentifierCollector {
    let mut visitor = IdentifierCollector::default();
    visit(src, &mut visitor);
    visitor.identifiers.sort();
    visitor
}

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
    assert_eq!(get_character_range('a', 'n'), visitor.identifiers);
}

#[test]
fn test_visit_expression_list() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a,b,c;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'c'), visitor.identifiers);
}

#[test]
fn test_if_statement() {
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
    assert_eq!(get_character_range('a', 'h'), visitor.identifiers);
}

#[test]
fn test_visit_for_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            FOR a := b TO c BY d DO
                e;
                f;
            END_FOR;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'f'), visitor.identifiers);
}

#[test]
fn test_visit_while_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            WHILE a < b DO
                c;
                d;
            END_WHILE;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}
#[test]
fn test_visit_repeat_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            REPEAT
                a;
                b;
            UNTIL c > d;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_case_statement() {
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
    assert_eq!(get_character_range('a', 'j'), visitor.identifiers);
}

#[test]
fn test_visit_multiplied_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            3(a+b);
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visist_array_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a[b];
            c[d,e+f];
            g[h+i][j+k];
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'k'), visitor.identifiers);
}

#[test]
fn test_visit_range_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a..b;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visit_assignment_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a := b;
            c => d;
            e =>;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'e'), visitor.identifiers);
}

#[test]
fn test_visit_cast_statement_expressions() {
    struct CastVisitor{
        visited: bool,
    }

    impl AstVisitor for CastVisitor{
        fn visit_cast_statement(&mut self, _stmt: &plc_ast::ast::CastStatement, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visited = CastVisitor{visited: false};

    visit(
        "
        PROGRAM prg
            INT#3;
        END_PROGRAM", &mut visited
    );
    
    assert!(visited.visited);
}


#[test]
fn test_visit_direct_access_statement_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            %IW1.2.3; 
            %MD4;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('1', '4'), visitor.identifiers);
}

#[test]
fn test_visit_call_statements() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a();
            b(c,d);
            e(f:=(g), h=>i);
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'i'), visitor.identifiers);
}

#[test]
fn test_visit_return_statement() {
    let visitor = collect_identifiers(
        "
        FUNCTION prg : INT
            RETURN a + b;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visit_into_var_global() {
    let visitor = collect_identifiers(
        "
        VAR_GLOBAL
            a : INT := c;
            c : INT := d;
        END_VAR"
    );
    assert_eq!(get_character_range('c', 'd'), visitor.identifiers);
}



#[test]
fn test_visit_data_type_declaration() {
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

    for st in &compilation_unit.implementations[0].statements {
        visitor.visit(st);
    }
    assert_eq!(6, visitor.count);
}

#[test]
fn test_visit_datatype_initializers_statement() {
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

    let mut expected = vec!["1", "3", "4", "7"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

    expected.extend(get_character_range('a', 'm'));
    assert_eq!(expected, visitor.identifiers);
}

#[test]
fn test_visit_array_declaration_statement() {
    let visitor = collect_identifiers(
        "
        TYPE MyArray: ARRAY[(a+b)..(c+d)] OF INT; END_TYPE",
    );
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_if_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            IF a THEN
                b;
            ELSIF c THEN
                d;
            ELSE
                e;
            END_IF;",
    );
    assert_eq!(get_character_range('a', 'e'), visitor.identifiers);
}

#[test]
fn test_for_loop_visting() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            FOR a := b TO c BY d DO
                e;
            END_FOR;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'e'), visitor.identifiers);
}

#[test]
fn test_while_loop_visiting() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            WHILE a DO
                b;
            END_WHILE;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);

    let visitor = collect_identifiers(
        "
        PROGRAM prg
            REPEAT
                a;
            UNTIL
                b = c
            END_REPEAT;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'c'), visitor.identifiers);
}

#[test]
fn test_case_stmt_visiting() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            CASE a OF
                b:
                    c;
                d, e:
                    f;
                ELSE
                    g;
            END_CASE;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'g'), visitor.identifiers);
}

#[test]
fn test_visit_qualified_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a.b;
            c.d^.e;
            f.g[h].i;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'i'), visitor.identifiers);
}


#[test]
fn test_visit_variable_block() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            VAR_INPUT
                a : INT := X;
            END_VAR
            VAR_OUTPUT
                b : INT := Y;
            END_VAR
        END_PROGRAM",
    );
    assert_eq!(get_character_range('X', 'Y'), visitor.identifiers);
}

#[test]
fn test_visit_continue_exit() {
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
    struct DefaultValueCollector {
        visited: bool,
    }

    // This is a simple visitor that collects all field names in a datatype
    impl AstVisitor for DefaultValueCollector {
        fn visit_default_value(&mut self, _stmt: &plc_ast::ast::DefaultValue, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visitor =  DefaultValueCollector{visited: false};
    
    visit(
        "
        VAR_GLOBAL CONSTANT
            a : INT;
        END_VAR
        ", 
        &mut visitor
    );
    assert!(visitor.visited);
}

#[test]
fn test_visit_direct_access() {
    struct Visited {
        visited: bool,
    }

    impl AstVisitor for Visited {
        fn visit_direct_access(&mut self, _stmt: &plc_ast::ast::DirectAccess, _node: &plc_ast::ast::AstNode) {
            self.visited = true;
        }
    }

    let mut visitor =  Visited{visited: false};
    
    visit(
        "
        PROGRAM prg
            x.1;
        ", 
        &mut visitor
    );
    assert!(visitor.visited);
}




