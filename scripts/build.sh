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
deb=0
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

# Create shared library from static library with platform-specific flags
function create_shared_library() {
    local cc=$1
    local lib_dir=$2
    local target=$3
    
    log "Creating a shared library from the compiled static library"
    
    # Check if we're on macOS and adjust linker flags accordingly
    case "$(uname -s)" in
        Darwin*)
            local cmd_args=("--shared" "-L$lib_dir" "-Wl,-force_load,$lib_dir/libiec61131std.a" "-o" "$lib_dir/libiec61131std.so" "-lm" "-framework" "CoreFoundation")
            if [[ -n "$target" ]]; then
                cmd_args+=("--target=$target")
            fi
            log "Running: $cc ${cmd_args[*]}"
            "$cc" "${cmd_args[@]}"
            ;;
        *)
            local cmd_args=("--shared" "-L$lib_dir" "-Wl,--whole-archive" "-liec61131std" "-o" "$lib_dir/libiec61131std.so" "-Wl,--no-whole-archive" "-lm" "-fuse-ld=lld")
            if [[ -n "$target" ]]; then
                cmd_args+=("--target=$target")
            fi
            log "Running: $cc ${cmd_args[*]}"
            "$cc" "${cmd_args[@]}"
            ;;
    esac
}

function run_package_std() {
    local cc
    cc=$(get_compiler)
    local OUTPUT_DIR=$project_location/output
    local target_dir="$project_location/target"
    local include_dir=$OUTPUT_DIR/include
    
    make_dir "$OUTPUT_DIR"
    log "Packaging Standard functions"
    log "Removing previous output folder"
    rm -rf "$OUTPUT_DIR"
    make_dir "$include_dir"
    
    # Copy the iec61131-st folder
    cp -r "$project_location"/libs/stdlib/iec61131-st/*.st "$include_dir"

    if [[ -n "$target" ]]; then
        for val in ${target//,/ }; do
            local lib_dir=$OUTPUT_DIR/$val/lib
            make_dir "$lib_dir"

            # Normalize target name for rustc
            local rustc_target=$val
            if [[ $val == *"-linux-gnu" && $val != *"unknown-linux-gnu" ]]; then
                rustc_target="${val/-linux-gnu/-unknown-linux-gnu}"
            fi
            
            # Determine release or debug directory
            local rel_dir="$target_dir/$rustc_target"
            if [[ $release -ne 0 ]]; then
                rel_dir="$rel_dir/release"
            else
                rel_dir="$rel_dir/debug"
            fi
            
            if [[ ! -d "$rel_dir" ]]; then
                echo "Compilation directory $rel_dir not found"
                exit 1
            fi
            
            cp "$rel_dir/"*.a "$lib_dir" 2>/dev/null || log "$rel_dir does not contain *.a files"
            create_shared_library "$cc" "$lib_dir" "$val"
        done
    else
        local lib_dir=$OUTPUT_DIR/lib
        make_dir "$lib_dir"
        
        # Determine release or debug directory
        local rel_dir="$target_dir"
        if [[ $release -ne 0 ]]; then
            rel_dir="$rel_dir/release"
        else
            rel_dir="$rel_dir/debug"
        fi
        
        cp "$rel_dir/"*.a "$lib_dir" 2>/dev/null || log "$rel_dir does not contain *.a files"
        create_shared_library "$cc" "$lib_dir" ""
    fi

    log "Enabling read/write on the output folder"
    chmod -R a+rw "$OUTPUT_DIR"

}

function get_project_version() {
    grep '^version' "$project_location/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

function target_to_deb_arch() {
    local t=$1
    case "$t" in
        x86_64*)  echo "amd64" ;;
        aarch64*) echo "arm64" ;;
        *)
            echo "Unsupported target architecture: $t"
            exit 1
            ;;
    esac
}

function target_to_multiarch_tuple() {
    local t=$1
    case "$t" in
        x86_64*)  echo "x86_64-linux-gnu" ;;
        aarch64*) echo "aarch64-linux-gnu" ;;
        *)
            echo "Unsupported target architecture: $t"
            exit 1
            ;;
    esac
}

function get_native_target() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64)  echo "x86_64-linux-gnu" ;;
        aarch64) echo "aarch64-linux-gnu" ;;
        *)
            echo "Unsupported native architecture: $arch"
            exit 1
            ;;
    esac
}

function build_lib_deb() {
    local target_val=$1
    local version=$2
    local deb_rev=$3
    local deb_output_dir=$4

    local deb_arch
    deb_arch=$(target_to_deb_arch "$target_val")
    local multiarch_tuple
    multiarch_tuple=$(target_to_multiarch_tuple "$target_val")
    local pkg_name="libiec61131std"
    local pkg_version="${version}-${deb_rev}"
    local stage_dir="$deb_output_dir/${pkg_name}_${pkg_version}_${deb_arch}"

    log "Building $pkg_name deb for $deb_arch ($target_val)"

    # Clean previous staging directory
    rm -rf "$stage_dir"

    # Create directory structure
    mkdir -p "$stage_dir/DEBIAN"
    mkdir -p "$stage_dir/usr/lib/$multiarch_tuple"
    mkdir -p "$stage_dir/usr/share/plc/include"
    mkdir -p "$stage_dir/usr/share/doc/$pkg_name"

    # Copy library files from output/
    local lib_source_dir
    if [[ -d "$project_location/output/$target_val/lib" ]]; then
        lib_source_dir="$project_location/output/$target_val/lib"
    elif [[ -d "$project_location/output/lib" ]]; then
        lib_source_dir="$project_location/output/lib"
    else
        echo "Error: No library output found for target $target_val"
        exit 1
    fi

    cp "$lib_source_dir/libiec61131std.so" "$stage_dir/usr/lib/$multiarch_tuple/" 2>/dev/null || true
    cp "$lib_source_dir/libiec61131std.a"  "$stage_dir/usr/lib/$multiarch_tuple/" 2>/dev/null || true

    # Verify at least the .so was copied
    if [[ ! -f "$stage_dir/usr/lib/$multiarch_tuple/libiec61131std.so" ]]; then
        echo "Error: libiec61131std.so not found in $lib_source_dir"
        exit 1
    fi

    # Copy .st include files
    cp "$project_location"/output/include/*.st "$stage_dir/usr/share/plc/include/"

    # Create copyright file with all license texts
    {
        echo "Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/"
        echo "Upstream-Name: RuSTy"
        echo "Upstream-Contact: https://github.com/PLC-lang/rusty"
        echo "Source: https://github.com/PLC-lang/rusty"
        echo ""
        echo "Files: *"
        echo "Copyright: 2020-2026, PLC-lang contributors"
        echo "License: LGPL-3.0-or-later"
        echo ""
        echo "License: LGPL-3.0-or-later"
    } > "$stage_dir/usr/share/doc/$pkg_name/copyright"
    cat "$project_location/COPYING.LESSER" >> "$stage_dir/usr/share/doc/$pkg_name/copyright"
    {
        echo ""
        echo "License: GPL-3.0"
    } >> "$stage_dir/usr/share/doc/$pkg_name/copyright"
    cat "$project_location/COPYING" >> "$stage_dir/usr/share/doc/$pkg_name/copyright"
    {
        echo ""
        echo "Files: libs/stdlib/*"
        echo "Copyright: 2020-2026, PLC-lang contributors"
        echo "License: LGPL-2.1"
        echo ""
        echo "License: LGPL-2.1"
    } >> "$stage_dir/usr/share/doc/$pkg_name/copyright"
    cat "$project_location/libs/stdlib/LICENSE" >> "$stage_dir/usr/share/doc/$pkg_name/copyright"

    # Calculate installed size in KiB
    local installed_size
    installed_size=$(du -sk "$stage_dir" | cut -f1)

    # Write DEBIAN/control
    cat > "$stage_dir/DEBIAN/control" <<EOF
Package: ${pkg_name}
Version: ${pkg_version}
Architecture: ${deb_arch}
Multi-Arch: same
Maintainer: PLC-lang Project
Section: libs
Priority: optional
Depends: libc6
Installed-Size: ${installed_size}
Homepage: https://github.com/PLC-lang/rusty
Description: IEC 61131-3 standard library for PLC
 Shared and static libraries implementing the IEC 61131-3 standard
 functions and function blocks for use with the PLC compiler.
 Includes standard library source (.st) files.
EOF

    # Write ldconfig triggers
    cat > "$stage_dir/DEBIAN/postinst" <<'SCRIPT'
#!/bin/sh
set -e
ldconfig
SCRIPT

    cat > "$stage_dir/DEBIAN/postrm" <<'SCRIPT'
#!/bin/sh
set -e
ldconfig
SCRIPT

    # Set permissions
    find "$stage_dir" -type d -exec chmod 0755 {} \;
    find "$stage_dir/usr" -type f -exec chmod 0644 {} \;
    chmod 0755 "$stage_dir/DEBIAN/postinst"
    chmod 0755 "$stage_dir/DEBIAN/postrm"

    # Build the .deb
    dpkg-deb --build --root-owner-group "$stage_dir" "$deb_output_dir/"
    log "Built: $deb_output_dir/${pkg_name}_${pkg_version}_${deb_arch}.deb"

    # Clean up staging directory
    rm -rf "$stage_dir"
}

function run_package_deb() {
    local version
    version=$(get_project_version)
    local deb_rev="1"
    local deb_output_dir="$project_location/target/debian"

    make_dir "$deb_output_dir"

    echo "Packaging Debian packages"
    echo "-----------------------------------"
    echo "Version: $version"

    # --- plc binary package via cargo-deb ---
    log "Building plc binary deb via cargo-deb"
    if command -v cargo-deb &> /dev/null; then
        cargo deb -p plc_driver --no-build --no-strip \
            --output "$deb_output_dir" \
            --deb-revision "$deb_rev"
        echo "plc binary deb built"
    else
        echo "Warning: cargo-deb not found, skipping plc binary deb"
        echo "Install with: cargo install cargo-deb"
    fi

    # --- libiec61131std library package via dpkg-deb ---
    if command -v dpkg-deb &> /dev/null; then
        if [[ -n "$target" ]]; then
            local built_archs=""
            for val in ${target//,/ }; do
                # Skip empty values (trailing commas)
                [[ -z "$val" ]] && continue
                # Deduplicate by deb architecture to avoid rebuilding the same .deb
                local arch
                arch=$(target_to_deb_arch "$val")
                if [[ "$built_archs" == *"$arch"* ]]; then
                    log "Skipping $val, already built deb for $arch"
                    continue
                fi
                built_archs="$built_archs $arch"
                build_lib_deb "$val" "$version" "$deb_rev" "$deb_output_dir"
            done
        else
            local native_target
            native_target=$(get_native_target)
            build_lib_deb "$native_target" "$version" "$deb_rev" "$deb_output_dir"
        fi
        echo "libiec61131std deb(s) built"
    else
        echo "Warning: dpkg-deb not found, skipping libiec61131std deb"
    fi

    echo "-----------------------------------"
    echo "Debian packages in: $deb_output_dir/"
    ls -la "$deb_output_dir/"*.deb 2>/dev/null || echo "No .deb files found"
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
    if [[ $deb -ne 0 ]]; then
        params="$params --deb"
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
LONGOPTS=sources,offline,release,check,check-style,build,doc,lit,test,junit,verbose,container,linux,container-engine:,container-name:,coverage,package,deb,target:

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
        --deb)
            deb=1
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

if [[ $deb -ne 0 ]]; then
    run_package_deb
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

if [[ -d "$project_location/target/" ]]; then
    log "Allow access to target folders"
    chmod -R a+rw "$project_location/target/"
fi

if [[ $offline -ne 0 ]]; then
    log "Removing temporary build directory : $BUILD_DIR"
    rm -rf "$BUILD_DIR"
fi

echo "Done"
echo "======================================"
