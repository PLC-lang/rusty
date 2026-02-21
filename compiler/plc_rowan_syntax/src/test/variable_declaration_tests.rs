use indoc::indoc;
use itertools::Itertools;
use plc_rowan_parser::grammar;

use crate::{
    ast::{VarDeclarationBlock, VarDeclarationBlocks},
    expect_all,
    test::test_util::parse_generic,
};

#[test]
fn parse_variable_declaration_blocks() {
    let text = indoc! {"
        VAR END_VAR
        VAR_INPUT END_VAR
        VAR_OUTPUT END_VAR
        VAR_IN_OUT END_VAR
        VAR_TEMP END_VAR
        VAR_GLOBAL END_VAR
        VAR_EXTERNAL END_VAR
        VAR_CONFIG END_VAR
        "};

    let vdb: VarDeclarationBlocks =
        parse_generic(text, grammar::pou_grammar::var_declaration_blocks).ok().unwrap();
    let mut blocks = vdb.var_declaration_blocks().into_iter();

    ["VAR", "VAR_INPUT", "VAR_OUTPUT", "VAR_IN_OUT", "VAR_TEMP", "VAR_GLOBAL", "VAR_EXTERNAL", "VAR_CONFIG"]
        .into_iter()
        .for_each(|kw| {
            let b = blocks.next().unwrap();
            assert_eq!(b.VarDeclarationType_token().unwrap().text(), kw);
            assert_eq!(b.var_declarations().count(), 0);
            assert_eq!(b.END_VAR_token().unwrap().text(), "END_VAR");
        });
}

#[test]
fn parse_variable_declrations() {
    let text = indoc! {"
        VAR
            x : INT;
            y : INT := 42;
            a,b : BOOL;
        END_VAR"
    };

    let block: VarDeclarationBlock =
        parse_generic(text, grammar::pou_grammar::var_declaration_block).ok().unwrap();

    let mut decls = block.var_declarations().into_iter();
    {
        let decl = decls.next().unwrap();
        let idents: Vec<_> = decl
            .identifier_list()
            .unwrap()
            .names()
            .map(|it| it.ident_token().unwrap().text().to_string())
            .collect_vec();
        assert_eq!(idents, vec!["x"]);

        assert_eq!(decl.type_ref().unwrap().ident_token().unwrap().text(), "INT");
        expect_all!(decl.colon_token(), decl.semicolon_token());
    }
    {
        let decl = decls.next().unwrap();
        let idents: Vec<_> = decl
            .identifier_list()
            .unwrap()
            .names()
            .map(|it| it.ident_token().unwrap().text().to_string())
            .collect_vec();
        assert_eq!(idents, vec!["y"]);

        assert_eq!(decl.type_ref().unwrap().ident_token().unwrap().text(), "INT");
        expect_all!(decl.colon_token(), decl.semicolon_token());
    }{
        let decl = decls.next().unwrap();
        let idents: Vec<_> = decl
            .identifier_list()
            .unwrap()
            .names()
            .map(|it| it.ident_token().unwrap().text().to_string())
            .collect_vec();
        assert_eq!(idents, vec!["a", "b"]);

        assert_eq!(decl.type_ref().unwrap().ident_token().unwrap().text(), "BOOL");
        expect_all!(decl.colon_token(), decl.semicolon_token());
    }
}
