use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn function_pointer_simple() {
    let result = codegen(
        r"
        FUNCTION echo : DINT
            VAR_INPUT
                value : INT;
            END_VAR

            echo := value;
        END_FUNCTION

        FUNCTION main
            VAR
                echoPtr : REF_TO echo;
            END_VAR

            echoPtr := REF(echo);
            echoPtr^(12345);
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r"");
}
