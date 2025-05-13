#!/bin/bash

# Adapted from: https://medium.com/@wujido20/handling-flags-in-bash-scripts-4b06b4d0ed04

# Function to display script usage
usage() {
 echo "Usage: $0 <pnpm_script> [OPTIONS]"
 echo "Options:"
 echo " -h, --help              Display this help message"
 echo " -b, --build             Build the docker image before running the container"
 echo " -m, --with-mqtt-broker  Run the MQTT broker along with the container"
}

# From Kshitij
if ! [ -x "$(command -v docker)" ]; then
  echo '[!] Error: docker is not installed.' >&2
  exit 1
fi
if ! [ -x "$(command -v docker-compose)" ]; then
  echo '[!] Error: docker-compose is not installed.' >&2
  exit 1
fi

# Default values for options
build=false
with_mqtt_broker=false

if [ -z "$1" ]; then
  echo "Missing argument: pnpm_script" >&2
  usage
  exit 1
fi

export PNPM_SCRIPT=$1
shift

# Function to handle options and arguments
handle_options() {
  while [ $# -gt 0 ]; do
    case $1 in
      -h | --help)
        usage
        exit 0
        ;;
      -b | --build)
        build=true
        ;;
      -m | --with-mqtt-broker)
        with_mqtt_broker=true
        ;;
      *)
        echo "Invalid option: $1" >&2
        usage
        exit 1
        ;;
    esac
    shift
  done
}

# Main script execution
handle_options "$@"

if [ "$build" = true ]; then
  docker-compose rm -v -f telemetry
  if [ "$with_mqtt_broker" = true ]; then
    command="docker-compose -f docker-compose.yml -f docker-compose.mqtt.yml up --build -V --abort-on-container-exit"
    echo "Running command: $command"
    $command
  else
    command="docker-compose up --build -V --abort-on-container-exit"
    echo "Running command: $command"
    $command
  fi
elif [ "$with_mqtt_broker" = true ]; then
  command="docker-compose -f docker-compose.yml -f docker-compose.mqtt.yml up --abort-on-container-exit"
  echo "Running command: $command"
  $command
else
  command="docker-compose up --abort-on-container-exit"
  echo "Running command: $command"
  $command
fi
