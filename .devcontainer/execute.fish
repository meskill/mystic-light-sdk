#! /usr/bin/env fish

while true
  set -l command  (cat .devcontainer/command-pipe)
  reset
  eval $command
end