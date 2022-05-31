use pretty_assertions::assert_eq;

use crate::{ast::GenericBinding, index::PouIndexEntry, test_utils::tests::index};

#[test]
fn generics_saved_in_index() {
    let (_, index) = index(
        r"
        FUNCTION foo<T: ANY> : T; END_FUNCTION
    ",
    );

    let foo_info = index.find_pou("foo").unwrap();
    assert!(foo_info.is_generic());
    if let PouIndexEntry::Function { generics, .. } = foo_info {
        let t = &generics[0];
        assert_eq!(
            &GenericBinding {
                name: "T".into(),
                nature: crate::ast::TypeNature::Any,
            },
            t
        );
    } else {
        panic!("{:#?} not a generic function", foo_info);
    }
}
