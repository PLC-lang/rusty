use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

macro_rules! assert_with_type_name {
    ($name:expr) => {
        let name = $name;
        let diagnostics = parse_and_validate(&format!("TYPE {name} : STRUCT x : DINT; END_STRUCT END_TYPE"));
        assert_validation_snapshot!(&diagnostics)
    };
}

#[test]
fn int() {
    assert_with_type_name!("INT");
}
