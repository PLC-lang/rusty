use crate::grammar::var_decl_grammar::var_declaration;
use crate::grammar::{name, name_ref};
use crate::parser::Parser;
use crate::SyntaxKind::{self, *};
use crate::T;

// program_declaration = 
//     'PROGRAM' identifier
//     { var_block }
//     [ statement_list ]
//     'END_PROGRAM' 

// (* --- FUNCTION --- *)
// function_declaration = 
//     'FUNCTION' identifier [ ':' type_reference ]
//     { var_block }
//     [ statement_list ]
//     'END_FUNCTION' 

// (* --- FUNCTION_BLOCK --- *)
//     'FUNCTION_BLOCK' { pou_modifier } identifier [ extends_clause ] [ implements_clause ]
//     { var_block }
//     { method_declaration | property_declaration }
//     [ statement_list ]
//     'END_FUNCTION_BLOCK' 

pub (crate) fn pou(p: &mut Parser) {
    let m = p.start();
    p.expect(POU_TYPE);
    name(p);
    if p.eat(T![:]) {
        p.expect(NAME_REF);
    }
    var_declaration_blocks (p);
    p.expect(POU_END_KEYWORD);
    p.eat(T![;]);
    m.complete(p, POU);
}

// VarDeclarationBlocks = VarDeclarationBlock*
fn var_declaration_blocks(p: &mut Parser) {
    let m = p.start();
    
    // Parse zero or more VarDeclarationBlock
    while p.at(VAR_DECLARATION_TYPE) {
        var_declaration_block(p);
    }
    
    m.complete(p, VAR_DECLARATION_BLOCKS);
}

// VarDeclarationBlock = VarDeclarationType VarDeclaration* 'END_VAR'
fn var_declaration_block(p: &mut Parser) {
    let m = p.start();
    p.expect(VAR_DECLARATION_TYPE);
    
    // Parse zero or more VarDeclaration
    while !p.at(END_VAR_KW) && !p.at(EOF) {
        if p.at(IDENT) {
            var_declaration(p);
        }else{
            p.err_and_bump("Expected a variable declaration (Identifier).");
        }
    }
    
    // Expect END_VAR
    p.expect(END_VAR_KW);
    
    m.complete(p, VAR_DECLARATION_BLOCK);
}





#[cfg(test)]
mod tests {
    use crate::{grammar::{compilation_unit, tests::{format_tree, parse_with}}, lexed_str::LexedStr};


    #[test]
    fn parse_pou_ok() {
        let input = r#"PROGRAM MyProgram
                                VAR         x : INT; END_VAR
                                VAR_INPUT   y : INT; END_VAR
                            END_PROGRAM"#;
        
        let input = LexedStr::new(input);
        let output = parse_with(&input, compilation_unit);
        insta::assert_snapshot!(format_tree(&output, &input));
    }
    
    #[test]
    fn test_pou_error() {
        let input = r#"PROGRAM MyProgram
                                VAR         
                                    x : INT /*missing semicolon*/ 
                                END_VAR
                            END_PROGRAM"#;
        
        let input = LexedStr::new(input);
        let output = parse_with(&input, compilation_unit);
        insta::assert_snapshot!(format_tree(&output, &input));
    }
}