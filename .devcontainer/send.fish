#! /usr/bin/env fish

echo (string replace $CONTAINER_PROJECT_HOME $HOST_PROJECT_HOME (realpath $argv)) > ./.devcontainer/command-pipe