// Malformed WSTRINGs must not crash the application; WSTRING_TO_* delegates to STRING_TO_* after a
// WSTRING->STRING conversion and parses the longest valid prefix instead.

// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    // WSTRING_TO_REAL / WSTRING_TO_LREAL: parse the valid prefix, default to 0.0
    printf('%f$N', WSTRING_TO_REAL(""));             // CHECK: 0.000000
    printf('%f$N', WSTRING_TO_REAL("asdf"));         // CHECK: 0.000000
    printf('%f$N', WSTRING_TO_REAL("1.2j3"));        // CHECK: 1.200000
    printf('%f$N', WSTRING_TO_REAL("12j3"));         // CHECK: 12.000000
    printf('%f$N', WSTRING_TO_REAL("123.456$R$N"));  // CHECK: 123.456001
    printf('%f$N', WSTRING_TO_LREAL(""));            // CHECK: 0.000000
    printf('%f$N', WSTRING_TO_LREAL("1.2j3"));       // CHECK: 1.200000

    // WSTRING_TO_DINT / WSTRING_TO_LINT: parse the valid prefix, default to 0
    printf('%d$N', WSTRING_TO_DINT(""));             // CHECK: 0
    printf('%d$N', WSTRING_TO_DINT("asdf"));         // CHECK: 0
    printf('%d$N', WSTRING_TO_DINT("12j3"));         // CHECK: 12
    printf('%d$N', WSTRING_TO_DINT("123.456"));      // CHECK: 123
    printf('%d$N', WSTRING_TO_LINT("12j3"));         // CHECK: 12
END_FUNCTION
