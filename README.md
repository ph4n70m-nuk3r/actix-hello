actix-hello
===========
__A simple web (REST) service built using the actix-web framework.__  
Note: This repository is an example project, please feel free to copy/fork/reuse anything within.
# DockerHub repository (GitHub Actions CI)
https://hub.docker.com/r/phantomnuker/actix-hello
# Local Development
## Update package dependencies (will cause breakages)
```sh
cargo update
```
## Build and Run from source  
```shell  
RUST_LOG='info,actix_hello=debug'  cargo run  --  ./config.toml
```
# Linux container development
## Run a pre-built (GitHub Actions) image from DockerHub
```shell
docker run \
    --name=actix-hello \
    -p 8080:8080 \
    -e RUST_LOG='info,actix_hello=debug' \
    -i -t \
    --rm \
    phantomnuker/actix-hello:muslrust-latest
```
## Build a local image from source (using clux/muslrust:stable)
```shell
docker build \
    ./ \
    -f Muslrust.Dockerfile \
    -t actix-hello:local
```
## Run image (runtime uses alpine:latest)
```shell
docker run \
    --name=actix-hello \
    -p 8080:8080 \
    -e RUST_LOG='info,actix_hello=debug' \
    -i -t \
    --rm \
    actix-hello:local
```
# Sending requests to the service (examples)
```shell
printf '%s\n' "$(curl -s http://localhost:8080/hi)"
```
output: `Hi!`
```shell
printf '%s\n' "$(curl -s http://localhost:8080/hello)"
```
output: `Hello!`
```shell
printf '%s\n' "$(curl -s http://localhost:8080/hello/your%20name%20here)"
```
output: `Hello your name here!`
```shell
printf '%s\n' "$(curl -d 'Message to repeat.' -s http://localhost:8080/echo)"
```
output: `Message to repeat.`