#!/bin/bash

# Parse parameters
vendor=0
offline=0
check=0
check_style=0
build=0
doc=0
test=0
lit=0
coverage=0
release=0
debug=0
container=0
container_engine=0
assume_linux=0
junit=0
package=0
target=""

CONTAINER_NAME='rust-llvm'

source "${BASH_SOURCE%/*}/common.sh"

function set_cargo_options() {
    CARGO_OPTIONS=""
    if [[ $debug -ne 0 ]]; then
        CARGO_OPTIONS="$CARGO_OPTIONS --verbose"
    fi
    if [[ $release -ne 0 ]]; then
        CARGO_OPTIONS="$CARGO_OPTIONS --release"
    fi
    if [[ $offline -ne 0 ]]; then
        CARGO_OPTIONS="$CARGO_OPTIONS --frozen"
    fi
    echo "$CARGO_OPTIONS"
}

function run_coverage() {
    log "Exporting Flags"
    export RUSTFLAGS="-C instrument-coverage"
    export LLVM_PROFILE_FILE="rusty-%p-%m.profraw"

    log "Cleaning before build"
    cargo clean
    log "Building coverage"
    cargo build --workspace
    log "Running coverage tests"
    cargo test --workspace
    log "Collecting coverage results"
    grcov . --binary-path ./target/debug/ -s . -t lcov --branch \
        --ignore "/*" \
        --ignore "src/main.rs" \
        --ignore "src/*/tests.rs" \
        --ignore "src/*/tests/*" \
        --ignore "tests/*" \
        --ignore "src/lexer/tokens.rs" \
        --ignore-not-existing -o lcov.info
}


function run_build() {
    CARGO_OPTIONS=$(set_cargo_options)

    # Run cargo build with release or debug flags
    echo "Build starting"
    echo "-----------------------------------"
    cmd="cargo build $CARGO_OPTIONS "
    log "Running $cmd"
    eval "$cmd"
    echo "-----------------------------------"
    if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
        echo "Build failed"
        exit 1
    else
        echo "Build done"
    fi
}

# Builds with set targets, useful for standard functions
function run_std_build() {
    CARGO_OPTIONS=$(set_cargo_options)

    # if the targets are set, we will build once per target

    # Run cargo build with release or debug flags
    echo "Build starting"
    echo "-----------------------------------"
    cmd="cargo build $CARGO_OPTIONS -p iec61131std"
    if [[ ! -z $target ]]; then
        for val in ${target//,/ }
        do
            # if the target ends with -linux-gnu but does not have unknown, add unknown
            if [[ $val == *"-linux-gnu" && $val != *"unknown-linux-gnu" ]]; then
                val="${val/-linux-gnu/-unknown-linux-gnu}"
            fi
            new_cmd="$cmd --target=$val"
            log "Running $new_cmd"
            eval "$new_cmd"
            echo "-----------------------------------"
            if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
                echo "Build $val failed"
                exit 1
            else
                echo "Build $val done"
            fi
        done
    else
        log "Running $cmd"
        eval "$cmd"
        echo "-----------------------------------"
        if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
            echo "Build failed"
            exit 1
        else
            echo "Build done"
        fi
    fi
}

function run_check() {
    CARGO_OPTIONS=$(set_cargo_options)
    log "Running cargo check"

    cargo check $CARGO_OPTIONS --workspace
}

function run_doc() {
    CARGO_OPTIONS=$(set_cargo_options)
    log "Running cargo doc --workspace $CARGO_OPTIONS"
    cargo doc --workspace $CARGO_OPTIONS
    log "Building book"
    log "Building preprocessor for the book"
    cargo build --release -p errorcode_book_generator
    cd book && mdbook build
    # test is disabled because not all files in the book exist. The pre-processor for error codes adds new files
    # mdbook test
}

function run_check_style() {
    CARGO_OPTIONS=$(set_cargo_options)
    log "Running cargo clippy"
    cargo clippy $CARGO_OPTIONS --workspace -- -Dwarnings
    log "Running cargo fmt check"
    cargo fmt -- --check
}

function run_lit_test() {
    # We need a binary as well as the stdlib and its *.so file before running lit tests
    run_build
    run_std_build
    run_package_std

    if [[ $release -eq 0 ]]; then
        lit -v -DLIB=$project_location/output -DCOMPILER=$project_location/target/debug/plc tests/lit/
    else
        lit -v -DLIB=$project_location/output -DCOMPILER=$project_location/target/release/plc tests/lit/
    fi
}

function run_test() {
    CARGO_OPTIONS=$(set_cargo_options)
    log "Running cargo test"
    if [[ $junit -ne 0 ]]; then
        #Delete the test results if they exist
        rm -rf "$project_location/test_results"
        make_dir "$project_location/test_results"
        # JUnit test should run via cargo-nextest
        log "cargo-nextest nextest run $CARGO_OPTIONS --lib --profile ci \
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/unit_tests.xml"
        cargo-nextest nextest run $CARGO_OPTIONS --lib --profile ci
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/unit_tests.xml

        # Run only the integration tests
        #https://stackoverflow.com/questions/62447864/how-can-i-run-only-integration-tests
        log "cargo-nextest nextest run $CARGO_OPTIONS --profile ci --test '*' \
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/integration_tests.xml "
        cargo-nextest nextest run $CARGO_OPTIONS --profile ci --test '*'
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/integration_tests.xml

        # Run the std integration
        log "cargo-nextest nextest run $CARGO_OPTIONS --profile ci -p iec61131std --test '*' \
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/std_integration_tests.xml"
        cargo-nextest nextest run $CARGO_OPTIONS --profile ci -p iec61131std --test '*'
        mv "$project_location"/target/nextest/ci/junit.xml "$project_location"/test_results/std_integration_tests.xml

    else
        cargo test $CARGO_OPTIONS --workspace
    fi
}

function generate_sources() {
    log "Collecting 3rd party sources"
    cargo vendor 3rd-party --versioned-dirs
    log "Packaging all sources into sources.zip"
    zip -r sources.zip ./ -x "output/*" -x "target/*" -x ".git/*" -q
}

function set_offline() {
    log "Vendor location set, using offline build"
    log "Copy the offline.toml config to build/.cargo/config.toml"
    make_dir "$BUILD_DIR"/.cargo
    cp "$project_location"/scripts/data/offline.toml "$BUILD_DIR"/.cargo/config.toml
    if [[ ! -d $project_location/3rd-party ]]; then
        echo "Offline sources not found at $project_location/3rd-party"
        exit 1
    fi
}

function run_package_std() {
    cc=$(get_compiler)
    OUTPUT_DIR=$project_location/output
    make_dir "$OUTPUT_DIR"
    log "Packaging Standard functions"
    log "Removing previous output folder"
    rm -rf $OUTPUT_DIR
    target_dir="$project_location/target"
    include_dir=$OUTPUT_DIR/include
    make_dir $include_dir
    #Copy the iec61131-st folder
    cp -r "$project_location"/libs/stdlib/iec61131-st/*.st "$include_dir"

    if [[ ! -z $target ]]; then
        for val in ${target//,/ }
        do
            lib_dir=$OUTPUT_DIR/$val/lib
            make_dir $lib_dir

            # if the target ends with -linux-gnu but does not have unknown, add unknown
            if [[ $val == *"-linux-gnu" && $val != *"unknown-linux-gnu" ]]; then
                rustc_target="${val/-linux-gnu/-unknown-linux-gnu}"
            fi
            rel_dir="$target_dir/$rustc_target"
            if [[ $release -ne 0 ]]; then
                rel_dir="$rel_dir/release"
            else
                rel_dir="$rel_dir/debug"
            fi
            if [[ ! -d "$rel_dir" ]]; then
                echo "Compilation directory $rel_dir not found"
                exit 1
            fi
            cp "$rel_dir/"*.a "$lib_dir" 2>/dev/null  || log "$rel_dir does not contain *.a files"
            # Create an SO file from the copied a file
            log "Creating a shared library from the compiled static library"
            log "Running : $cc --shared -L$lib_dir \
                -Wl,--whole-archive -liec61131std \
                -o $lib_dir/out.so -Wl,--no-whole-archive \
                -lm \
                -fuse-ld=lld \
                --target=$val"
            $cc --shared -L"$lib_dir" \
                -Wl,--whole-archive -liec61131std \
                -o "$lib_dir/out.so" -Wl,--no-whole-archive \
                -lm \
                -fuse-ld=lld \
                --target="$val"

            mv "$lib_dir/out.so" "$lib_dir/libiec61131std.so"
        done
    else
        lib_dir=$OUTPUT_DIR/lib
        make_dir $lib_dir
        if [[ $release -ne 0 ]]; then
            rel_dir="$target_dir/release"
        else
            rel_dir="$target_dir/debug"
        fi
        cp "$rel_dir/"*.a "$lib_dir" 2>/dev/null || log "$rel_dir does not contain *.a files"
        # Create an SO file from the copied a file
        log "Creating a shared library from the compiled static library"
        log "Running : $cc --shared -L"$lib_dir" \
            -Wl,--whole-archive -liec61131std \
            -o "$lib_dir/out.so" -Wl,--no-whole-archive \
            -lm \
            -fuse-ld=lld "
        $cc --shared -L"$lib_dir" \
            -Wl,--whole-archive -liec61131std \
            -o "$lib_dir/out.so" -Wl,--no-whole-archive \
            -lm \
            -fuse-ld=lld
        mv "$lib_dir/out.so" "$lib_dir/libiec61131std.so"
    fi

    log "Enabling read/write on the output folder"
    chmod a+rw $OUTPUT_DIR -R

}

function run_in_container() {
    if [ "$container_engine" == "0" ]; then
        container_engine=$(get_container_engine)
    fi
    params=""
    options=""

    if [[ $offline -ne 0 ]]; then
        params="$params --offline"
    fi
    if [[ $debug -ne 0 ]]; then
        params="$params --verbose"
    fi
    if [[ $check -ne 0 ]]; then
        params="$params --check"
    fi
    if [[ $check_style -ne 0 ]]; then
        params="$params --check-style"
    fi
    if [[ $build -ne 0 ]]; then
        params="$params --build"
    fi
    if [[ $release -ne 0 ]]; then
        params="$params --release"
    fi
    if [[ $coverage -ne 0 ]]; then
        params="$params --coverage"
    fi
    if [[ $test -ne 0 ]]; then
        params="$params --test"
    fi
    if [[ $lit -ne 0 ]]; then
        params="$params --lit"
    fi
    if [[ $junit -ne 0 ]]; then
        params="$params --junit"
    fi
    if [[ $doc -ne 0 ]]; then
        params="$params --doc"
    fi
    if [[ $package -ne 0 ]]; then
        params="$params --package"
    fi
    if [[ ! -z $target ]]; then
        params="$params --target $target"
    fi

    volume_target="/build"
    unameOut="$(uname -s)"
    case "${unameOut}" in
        Linux*)
            volume_target="/build"
            ;;
        MINGW* | cygwin*)
            volume_target="C:\\\\build"
            ;;
        *)
            echo "Unsupported os $unameOut"
            exit 1
    esac

    build_location=$(sanitize_path "$project_location")
    log "Sanitized Project location : $project_location"

    command_to_run="$container_engine run $options -v $build_location:$volume_target $CONTAINER_NAME scripts/build.sh $params"
    log "Running command : $command_to_run"
    eval "$command_to_run"
}

# More safety, by turning some bugs into errors.
set -o errexit -o pipefail -o noclobber -o nounset

OPTIONS=sorbvc
LONGOPTS=sources,offline,release,check,check-style,build,doc,lit,test,junit,verbose,container,linux,container-engine:,container-name:,coverage,package,target:

check_env
# -activate quoting/enhanced mode (e.g. by writing out “--options”)
# -pass arguments only via   -- "$@"   to separate them correctly
! PARSED=$(getopt --options="$OPTIONS" --longoptions="$LONGOPTS" --name "$0" -- "$@")
if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
    exit 2
fi

# read getopt’s output this way to handle the quoting right:
eval set -- "$PARSED"

while true; do
    case "$1" in
        -s|--sources)
            vendor=1
            ;;
        -o|--offline)
            offline=1
            ;;
        -r|--release)
            release=1
            ;;
        -v|--verbose)
            debug=1
            ;;
        -c|--container)
            container=1
            ;;
        --container-engine)
            shift;
            container_engine=$1
            ;;
        --container-name)
            shift;
            CONTAINER_NAME=$1
            ;;
        --linux)
            assume_linux=1
            ;;
        --check-style)
            check_style=1
            ;;
        --doc)
            doc=1
            ;;
        --check)
            check=1
            ;;
        -b|--build)
            build=1
            ;;
        --test)
            test=1
            ;;
        --lit)
            lit=1
            ;;
        --junit)
            junit=1
            ;;
        --coverage)
            coverage=1
            ;;
        --package)
            package=1
            ;;
        --target)
            shift
            target=$1
            ;;
        --)
            shift
            break
            ;;
        *)
            echo "Programming error"
            exit 3
            ;;
    esac
    shift
done

project_location=$(find_project_root)
log "Moving to project level directory $project_location"
cd "$project_location"

if [[ $container -ne 0 ]]; then
    log "Container Build"
    run_in_container
    exit 0
fi

if [[ $vendor -ne 0 ]]; then
    generate_sources
    exit 0
fi

if [[ $offline -ne 0 ]]; then
    BUILD_DIR=$project_location/build
    make_dir "$BUILD_DIR"
    set_offline
    log "Moving into $BUILD_DIR"
    cd "$BUILD_DIR"
fi

if [[ $check -ne 0 ]]; then
    run_check
fi

if [[ $check_style -ne 0 ]]; then
    run_check_style
fi

if [[ $build -ne 0 ]]; then
    run_build
    #Build the standard functions
    run_std_build
fi

if [[ $package -ne 0 ]]; then
    run_package_std
fi

if [[ $test -ne 0 ]]; then
    run_test
fi

if [[ $lit -ne 0 ]]; then
    run_lit_test
fi

if [[ $doc -ne 0 ]]; then
    run_doc
fi

if [[ $coverage -ne 0 ]]; then
    run_coverage
fi

if [[ -d $project_location/target/ ]]; then
    log "Allow access to target folders"
    chmod a+rw -R $project_location/target/
fi

if [[ $offline -ne 0 ]]; then
    log "Removing temporary build directory : $BUILD_DIR"
    rm -rf "$BUILD_DIR"
fi

echo "Done"
echo "======================================"
