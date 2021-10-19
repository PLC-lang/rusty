#!/bin/bash

# Parse parameters
vendor=0
offline=0
check=0
build=0
release=0
debug=0
container=0
build_container=0
assume_linux=0

source "${BASH_SOURCE%/*}/common.sh"

function set_cargo_options() {
	CARGO_OPTIONS=""

	if [[ $debug -ne 0 ]]; then
		CARGO_OPTIONS="$CARGO_OPTIONS --verbose"
	fi
	CARGO_OPTIONS=""
	if [[ $release -ne 0 ]]; then
		CARGO_OPTIONS="$CARGO_OPTIONS --release"
	fi
	if [[ $offline -ne 0 ]]; then
		CARGO_OPTIONS="$CARGO_OPTIONS --frozen"
	fi
	echo "$CARGO_OPTIONS"
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

function check() {
	CARGO_OPTIONS=$(set_cargo_options)
  log "Running cargo clippy"
	cargo clippy $CARGO_OPTIONS -- -Dwarnings
  log "Running cargo fmt check"
	cargo fmt -- --check
	cargo test $CARGO_OPTIONS
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

function run_in_container() {
	container_engine=$(get_container_engine)
	params=""
	if [[ $offline -ne 0 ]]; then
		params="$params --offline"
	fi
	if [[ $debug -ne 0 ]]; then
		params="$params --verbose"
	fi
	if [[ $check -ne 0 ]]; then
		params="$params --check"
	fi
	if [[ $build -ne 0 ]]; then
		params="$params --build"
	fi
	if [[ $release -ne 0 ]]; then
		params="$params --release"
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

	command_to_run="$container_engine run -it -v $build_location:$volume_target rust-llvm  scripts/build.sh $params"
	log "Running command : $command_to_run"
	eval "$command_to_run"
}

function build_docker_file() {
	container_engine=$(get_container_engine)
	if [[ $assume_linux -ne 0 ]]; then
		os=linux
	else
		unameOut="$(uname -s)"
		case "${unameOut}" in
				Linux*)
						os=linux
						;;
				MINGW* | cygwin*)     
						os=windows
						;;
		esac
	fi
	docker_file_location="$project_location/docker-build/$os"
	$container_engine build "$docker_file_location" -t rust-llvm
}


# More safety, by turning some bugs into errors.
set -o errexit -o pipefail -o noclobber -o nounset

OPTIONS=sorbvc
LONGOPTS=sources,offline,release,check,build,verbose,container,build-container,linux

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
					shift
					;;
			-o|--offline)
					offline=1
					shift 
					;;
			-r|--release)
					release=1
					shift
					;;
			-v|--verbose)
					debug=1
					shift
					;;
			-c|--container)
					container=1
					shift
					;;
			--build-container)
					build_container=1
					shift
					;;
			--linux)
					assume_linux=1
					shift
					;;
			--check)
				  check=1
					shift
					;;
			-b|--build)
				  build=1
					shift
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
done

project_location=$(find_project_root)
log "Moving to project level directory $project_location"
cd "$project_location"


if [[ $container -ne 0 ]]; then
	log "Container Build"
	run_in_container
	exit 0
fi

if [[ $build_container -ne 0 ]]; then
	build_docker_file
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
  check
	if [[ $build -ne 0 ]]; then
		run_build
	fi
	exit 0
fi

if [[ $build -ne 0 ]]; then
  run_build
	exit 0
fi

log "Removing temporary build directory : $BUILD_DIR"
rm -rf "$BUILD_DIR"


echo "Done"
echo "======================================"
