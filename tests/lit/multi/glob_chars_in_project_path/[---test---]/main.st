FUNCTION main : DINT
    // `main.st` is resolved as a literal path while `src/helper.st` is pulled
    // in via the `src/*.st` glob -- both relative to a project directory whose
    // name contains glob metacharacters (`[`, `]`). If path resolution regresses
    // to feeding the bracketed location straight into glob(), `helper` is never
    // compiled and this fails to link.
    printf('result=%d$N', helper());
    // CHECK: result=42
    main := 0;
END_FUNCTION
