#!/usr/bin/env bash

SERVICES=(api_gateway data_persistor shorts_service)

#
# Function to help with script usage
#
usage() {
  echo "Build one or more docker images"
  echo "--------------------------------------------------------------"
  echo "Available arguments:"
  echo "  -o/--only-push  Skip building step and push available images"
  echo "  -p/--dont-push  Disables pushing images"
  echo "  -s/--service    Build image for specific service(s)"
  echo "                    If not specified, all images are built"
  echo "  -h/--help       Shows this message"
}

#
# Function to build docker image
#
# Arguments
#   1: service_name
#   2: service version
#
build_image() {
  if [ ! $# -eq 2 ]; then
    echo "Not enough arguments passed to build image"
    return 1
  fi

  local _service="$1"
  local _version="$2"

  echo "Building image $_service v$_version with"
  docker build --build-arg SERVICE_NAME="$_service" -t registry.msvacina.cz/sas-"$_service":"$_version" -t registry.msvacina.cz/sas-"$_service":latest ./
}

#
# Function to push images to repository
#
# Arguments:
#   1: service_name
#   2: service_version (optional)
#
push_image() {
  if [ ! $# -ge 1 ]; then
    echo "Not enough arguments passed to push image"
    return 1
  fi

  local _service_name="$1"

  echo "Pushing image for $_service_name"
  docker push -a registry.msvacina.cz/sas-"$_service_name" >/dev/null

  if [ $? -eq 0 ]; then
    echo "Image $_service_name pushed successfully"
  fi
}

main() {
  local _services=()
  local _push_only=false
  local _dont_push=false
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -s|--service)
        while [[ ! "$2" = "-*" ]] && [ -n "$2" ]; do
          _services+=("$2")
          shift
        done
        shift
        ;;
      -o|--push_only)
        _push_only=true
        shift
        ;;
      -p|--dont-push)
        _dont_push=true
        shift
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        echo "Unknown argument '$1'"
        echo
        usage
        exit 1
        ;;
    esac
  done

  if [ "$_push_only" == true ] && [ "$_dont_push" == true ]; then
    echo "Colliding arguments passed - -push-only and -dont-push > No idea what to do"
    exit 2
  fi

  if [ -z "${_services[0]}" ]; then
    _services=${SERVICES[*]}
  fi

  echo "Building services: ${_services[*]}"
  for service in ${_services[@]}; do
    service_version=$(grep -Eo '^version = ".*"' "$service/Cargo.toml" 2>/dev/null | cut -d\" -f2)

    if [ -z "$service_version" ]; then
      echo "Cannot find version of $service... skipping"
      continue
    fi

    if [ "$_push_only" == false ]; then
      build_image "$service" "$service_version"
    fi

    if [ "$_dont_push" == false ]; then
      push_image "$service"
    fi
  done
}

main "$@"
