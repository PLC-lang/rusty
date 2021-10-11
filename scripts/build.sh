#!/bin/bash
# Build script for rusty

# Parse parameters
vendor=0
offline=0
check=0
release=0
debug=0
package=0
container=0

debug=0

machine=None
unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)     machine=Linux;;
    MINGW*)     machine=Windows;;
esac

if [ "{$machine}" == "None" ]; then
				log "Unkown system : $unameOut"
				exit 1
fi


function log() {
	if [[ $debug -ne 0 ]]; then
		echo $1
	fi
}

function make_dir() {
if [[ ! -d $1 ]]; then
	log "Creating a target build directory at $1"
	mkdir $1
fi
}

function check_env() {
	# -allow a command to fail with !’s side effect on errexit
	# -use return value from ${PIPESTATUS[0]}, because ! hosed $?
	! getopt --test > /dev/null 
	if [[ ${PIPESTATUS[0]} -ne 4 ]]; then
			echo 'Error:  extended getopts needed'
			exit 1
	fi
}


function build() {
	original_location=$PWD
	CARGO_OPTIONS=""

	if [[ $debug -ne 0 ]]; then
		CARGO_OPTIONS="$CARGO_OPTIONS --verbose"
	fi
	BUILD_DIR=$project_location/build
	make_dir $BUILD_DIR
	log "Moving into $BUILD_DIR"
	cd $BUILD_DIR

	CARGO_OPTIONS=""
	if [[ $package -ne 0 ]]; then
		make_dir output
		CARGO_OPTIONS="$CARGO_OPTIONS --target-dir=$BUILD_DIR/target"
	fi
	if [[ $release -ne 0 ]]; then
		CARGO_OPTIONS="$CARGO_OPTIONS --release"
	fi
	if [[ $offline -ne 0 ]]; then
		set_offline
		CARGO_OPTIONS="$CARGO_OPTIONS --frozen"
	fi

	# Run cargo build with release or debug flags
	echo "Build starting"
	echo "-----------------------------------"
	cmd="cargo build $CARGO_OPTIONS " 
	log "Running $cmd" 
	eval $cmd
	echo "-----------------------------------"
	if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
		echo "Build failed"
		exit 1
	else
		echo "Build done"
	fi


}

function generate_sources() {
	log "Collecting 3rd party sources"
	cargo vendor 3rd-party --versioned-dirs
}

function set_offline() {
	log "Vendor location set, using offline build"
	log "Copy the offline.toml config to .cargo/config.toml"
	make_dir .cargo
	cp $project_location/scripts/data/offline.toml .cargo/config.toml
	if [[ ! -d $project_location/3rd-party ]]; then
		echo "Offline sources not found at $project_location/3rd-party"
		exit 1
	fi
}

function build_book() {
	book_args=""
	if [[ $package -ne 0 ]]; then
		book_args="-d $BUILD_DIR/output/book"
	fi
	# Compile the book
	log "Building book"
	log "-----------------------------------"
	mdbook build $book_location $book_args
	if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
		echo "Book Build failed"
		exit 1
	else 
		log "Done buinding book"
		log "-----------------------------------"
	fi
}

function package_build() {
	log "Copying build artifact output directory"
	folder=debug
	if [[ $release -ne 0 ]]; then
		folder=release
	fi
	cp $BUILD_DIR/target/$folder/rustyc $BUILD_DIR/output/
	echo "Output saved in $BUILD_DIR/output"
}

function build_in_container() {
	log "Trying docker"
	if command -v docker &> /dev/null 
	then
		container_engine=docker
	else
		log "Docker not found, trying podman"
	  if command -v podman &> /dev/null 
	  then
	  	container_engine=podman
	  else
		  echo "Docker or podman not found"
		  exit 1
	  fi
	fi
	log "container engine found at : $container_engine"
	params=""
	if [[ $package -ne 0 ]]; then
		params="$params --package"
	fi
	if [[ $offline -ne 0 ]]; then
		params="$params --offline"
	fi
	if [[ $debug -ne 0 ]]; then
		params="$params --verbose"
	fi
	if [[ $release -ne 0 ]]; then
		params="$params --release"
	fi

	volume_target="/build"
	if [ "$machine" == "Windows" ]; then
					log "Running on Windofs, setting build directory to C:\Build"
					volume_target="C:\\build"
	fi

	$container_engine run -it -v $project_location:$volume_target rust-llvm  scripts/build.sh $params

}

# More safety, by turning some bugs into errors.
set -o errexit -o pipefail -o noclobber -o nounset


OPTIONS=sorvpc
LONGOPTS=sources,offline,release,verbose,package,container

check_env
# -activate quoting/enhanced mode (e.g. by writing out “--options”)
# -pass arguments only via   -- "$@"   to separate them correctly
! PARSED=$(getopt --options=$OPTIONS --longoptions=$LONGOPTS --name "$0" -- "$@")
if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
		# e.g. return value is 1
		#  then getopt has complained about wrong arguments to stdout
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
			-p|--package)
					package=1
					shift
					;;
			-c|--container)
					container=1
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

log "Locating project root"
project_location=`cargo locate-project --message-format plain`
project_location=`dirname $project_location`
book_location=$project_location/book
log "Project root at $project_location"
log "Book location at $book_location"
log "Moving to project level directory $project_location"
cd $project_location


if [[ $container -ne 0 ]]; then
	log "Container Build"
	build_in_container
	exit 0
fi

if [[ $vendor -ne 0 ]]; then
	generate_sources
	exit 0
fi


build
build_book

if [[ $package -ne 0 ]]; then
	package_build
fi
echo "Done"
echo "======================================"
