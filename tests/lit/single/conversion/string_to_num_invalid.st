// Malformed strings must not crash the application.

// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    // STRING_TO_REAL / STRING_TO_LREAL: parse the valid prefix, default to 0.0
    printf('%f$N', STRING_TO_REAL(''));             // CHECK: 0.000000
    printf('%f$N', STRING_TO_REAL('asdf'));         // CHECK: 0.000000
    printf('%f$N', STRING_TO_REAL('1.2j3'));        // CHECK: 1.200000
    printf('%f$N', STRING_TO_REAL('12j3'));         // CHECK: 12.000000
    printf('%f$N', STRING_TO_REAL('123.456$R$N'));  // CHECK: 123.456001
    printf('%f$N', STRING_TO_LREAL(''));            // CHECK: 0.000000
    printf('%f$N', STRING_TO_LREAL('1.2j3'));       // CHECK: 1.200000

    // STRING_TO_DINT / STRING_TO_LINT: reject malformed input, default to 0
    printf('%d$N', STRING_TO_DINT(''));             // CHECK: 0
    printf('%d$N', STRING_TO_DINT('asdf'));         // CHECK: 0
    printf('%d$N', STRING_TO_DINT('12j3'));         // CHECK: 0
    printf('%d$N', STRING_TO_DINT('123.456'));      // CHECK: 123
    printf('%d$N', STRING_TO_LINT('12j3'));         // CHECK: 0
END_FUNCTION
