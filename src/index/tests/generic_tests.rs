use pretty_assertions::assert_eq;

use crate::{ast::GenericBinding, test_utils::tests::index, typesystem::DataTypeInformation};

#[test]
fn generics_saved_in_index() {
    let (_,index) = index(r"
        FUNCTION foo<T: ANY> : T; END_FUNCTION
    ");

    let foo_info = index.find_effective_type_info("foo").unwrap();
    assert!(foo_info.is_generic());
    if let DataTypeInformation::Struct{ generics, .. } = foo_info {
        let t = &generics[0];
        assert_eq!(&GenericBinding{ name: "T".into(), nature: "ANY".into()}, t);
    } else {
        panic!("{:#?} not a struct", foo_info);
    }
}
