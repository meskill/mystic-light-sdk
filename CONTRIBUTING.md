# Contributing

Make sure you have Rust toolchain installed

## Setup

## Linux

**You won't be able to execute program or run tests as sdk supports only windows**

1. Add windows target `rustup target add x86_64-pc-windows-gnu`
2. Add MinGW (e.g. for the alpine `apk add mingw-w64-gcc`)
3. Specify target by env `CARGO_BUILD_TARGET=x86_64-pc-windows-gnu`

You can now build some of the examples using `cargo build --example disable_light_for_5_sec`, but you have to run it on any other windows machine with the Dragon Center installed.


## Windows

Should work without any hassle.

Run `cargo build --example disable_light_for_5_sec` to build example and `cargo test` to run tests

### WSL 2

There is a catch - wsl 2 runs in linux environment, but it still can run windows executable, so we can use WSL 2 linux environment to build actual program and then run built .exe file

1. Install additional libs to able to build windows app following [linux instructions](#linux)
2. Run `cargo test`

Run `cargo build --example disable_light_for_5_sec` to build example, but you still required to run it outside WSL as it requires Dragon Center to communicate


### WSL 2 + Docker

If you are using Rust inside Docker container you not able to run .exe files from inside it, but you can use named pipe to pass commands to the host WSL 2 instance that able to run .exe

For this either use Remote Containers extension with the provided `.devcontainer` setup or follow next instruction

1. Install additional libs to able to build windows app following [linux instructions](#linux)
2. Mount existing named pipe and scripts to the docker container `type=bind,src=${localWorkspaceFolder}/.devcontainer,dst=${containerWorkspaceFolder}/.devcontainer`
3. Specify environment variables for container `CONTAINER_PROJECT_HOME` - path to the project inside the container, `HOST_PROJECT_HOME` - path to projects in WSL
4. Run the container
5. You can use `wsl2-docker.env` in order fill needed environment variable that will wrap execution for the program run
6. Inside WSL start script `.devcontainer/execute.fish` that will listen for new commands and execute it
7. Run `cargo test`
8. Execution will end without any info, but you should see test running output in the WSL terminal


Run `cargo build --example disable_light_for_5_sec` to build example, but you still required to run it outside WSL as it requires Dragon Center to communicate