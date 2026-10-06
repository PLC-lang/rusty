// `-V` prints the name and the package version on one line.
// RUN: %PLC -V | %CHECK --check-prefix=SHORT %s
// SHORT: plc {{[0-9]+\.[0-9]+\.[0-9]+.*}}

// `--version` adds the git and build details on the lines below. It needs no input files.
// RUN: %PLC --version | %CHECK --check-prefix=LONG %s
// LONG: plc {{[0-9]+\.[0-9]+\.[0-9]+.*}}
// LONG-NEXT: branch:{{.*}}
// LONG-NEXT: commit_hash:{{.*}}
// LONG-NEXT: build_time:{{.*}}
// LONG-NEXT: build_env:{{.*}}
