#! /usr/bin/env fish

echo (string replace $CONTAINER_PROJECT_HOME $HOST_PROJECT_HOME $argv) > .devcontainer/command-pipe