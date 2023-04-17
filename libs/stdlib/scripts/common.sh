#!/bin/bash

debug=0
function log() {
	if [[ $debug -ne 0 ]]; then
		>&2 echo "$1"
	fi
}

function make_dir() {
if [[ ! -d $1 ]]; then
	log "Creating a directory at $1"
	mkdir -p "$1"
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

function get_compiler() {
	log "Trying clang"
	res=
	if command -v clang &> /dev/null
	then
		log "Found clang, using as default"
		res=clang
	else
		log "Trying clang-14"
		if command -v clang-14 &> /dev/null
		then
			log "Found clang, using as default"
			res=clang-14
		else 
			echo 'Error : clang / clang-14 not found'
			exit 1
		fi
	fi
	log "Compiler found : $res"
	echo $res
}

function get_container_engine() {
	log "Trying docker" 
	if command -v docker &> /dev/null 
	then
		container_engine=docker
	else
		>&2 log "Docker not found, trying podman"
	  if command -v podman &> /dev/null 
	  then
	  	container_engine=podman
	  else
		  echo "Docker or podman not found"
		  exit 1
	  fi
	fi
	log "container engine found : $container_engine"
	echo $container_engine
}


function find_project_root() {
	log "Locating project root"
	if command -v cargo &> /dev/null 
	then
		log "Using cargo"
		project_location=$(cargo locate-project --message-format plain)
		project_location=$(dirname "$project_location")
	else
		log "Cargo not found, using script location"
		project_location="${BASH_SOURCE%/*/..}"
		project_location=$(dirname $(readlink -f "$project_location"))
	fi
	log "Found project location at $project_location"
	# echo $project_location | tr -t "\\" "\\\\"
	echo $project_location
}

function sanitize_path() {
	target=$1
	echo "${target//\\/\\\\}"
}
