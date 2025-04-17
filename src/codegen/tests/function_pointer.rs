use test_utils::codegen;

#[test]
fn demo() {
    let result = codegen(
        r"
        FUNCTION prototype : DINT
            VAR_INPUT
                a : DINT;
            END_VAR
        END_FUNCTION

        FUNCTION test : DINT
            VAR
                f : REF_TO prototype := REF(prototype);
            END_VAR

            f^(1);
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define i32 @prototype(i32 %0) {
    entry:
      %prototype = alloca i32, align 4
      %a = alloca i32, align 4
      store i32 %0, i32* %a, align 4
      store i32 0, i32* %prototype, align 4
      %prototype_ret = load i32, i32* %prototype, align 4
      ret i32 %prototype_ret
    }

    define i32 @test() {
    entry:
      %test = alloca i32, align 4
      %f = alloca i32 (i32)**, align 8
      store i32 (i32)** bitcast (i32 (i32)* @prototype to i32 (i32)**), i32 (i32)*** %f, align 8
      store i32 0, i32* %test, align 4
      store i32 (i32)** bitcast (i32 (i32)* @prototype to i32 (i32)**), i32 (i32)*** %f, align 8
      %call = call i32 @prototype(i32 1)
      %test_ret = load i32, i32* %test, align 4
      ret i32 %test_ret
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "#);
}
