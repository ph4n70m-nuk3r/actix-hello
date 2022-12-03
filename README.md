actix-hello
===========
Simple web (REST) service built using the actix-web framework.
# Local Development
## Update package dependencies  
```sh
cargo update
```
## Build and Run from source  
```shell  
cargo run  --  ./config.toml
```
# Send requests to the service (example)  
```sh  
$ echo "$(curl -s http://localhost:8080/hi)"
Hi!

$ echo "$(curl -s http://localhost:8080/hello)"
Hello!

$ echo "$(curl -s http://localhost:8080/hello/<name>)"
Hello <name>!

$ echo "$(curl -d 'Message to repeat.' -s http://localhost:8080/echo)"
Message to repeat.
```
# Docker development (Build uses clux/muslrust, Runtime uses alpine:latest)
## Build image
```shell
docker build . -f Muslrust.Dockerfile -t actix-hello:latest
```
## Run image
```shell
docker run --network=host -it --rm actix-hello:latest config.toml
```
