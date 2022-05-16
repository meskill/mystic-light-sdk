#! /usr/bin/env fish

echo "Run any cargo command"

while true
  set path (cat .devcontainer/command-pipe)

  kill %1
  reset

  set previous_path $path

  eval "$path &"
end