# Simple GRPC Hello Service Server/Client with Rust
A simple GRPC service to show how to use GRPC with Rust.<br>
You can use it as a template for GRPC project.

## Build project
```bash
$ cargo build --bins
```

## Run GRPC server
In a terminal:
```bash
$ cargo run --bin hello-server
```
## Run GRPC client
In another terminal
```bash
$ cargo run --bin hello-client
  Compiling rust-grpc-hello v0.1.0 (/mnt/workspace/rust/rust-grpc-hello)
    Finished dev [unoptimized + debuginfo] target(s) in 2.68s
     Running `target/debug/hello-client`
message: "Hello World"
```
## Static GRPC server binary build 
### Build Rust with musl Docker image
Use Alpine linux to build a Rust environment with musl
```bash
$ # sh rust-musl.sh
$ docker build -t rust:musl -f Dockerfile.rust-musl . 
$ docker image ls
rust                musl                1bead3b504b3        11 minutes ago      430MB
helloclient         cxx                 9d88c6f61ff5        5 days ago          2.31MB
helloserver         cxx                 e5d84a5c11f1        5 days ago          2.45MB
hello               cxx                 10ac699c3fa0        6 days ago          1.82MB
alpine              3.10                961769676411        3 weeks ago         5.58MB
alpine              edge                70997d35b3ed        4 weeks ago         5.59MB
```
### Build static GRPC server and client
Use the previous Docker image, to build a release static binaries for server and client
```bash
$ # sh musl-build.sh
$ docker run -it -v $PWD:/build rust:musl
$ ls -l target/x86_64-alpine-linux-musl/release/hello-*
-rwxr-xr-x 2 root root 5878792 Sep 12 23:46 target/x86_64-alpine-linux-musl/release/hello-client
-rw-r--r-- 1 root root     177 Sep 12 23:46 target/x86_64-alpine-linux-musl/release/hello-client.d
-rwxr-xr-x 2 root root 6032944 Sep 12 23:46 target/x86_64-alpine-linux-musl/release/hello-server
-rw-r--r-- 1 root root     177 Sep 12 23:46 target/x86_64-alpine-linux-musl/release/hello-server.d
```
### Build a Server Docker image
```bash
$ docker build -t hello-server:rust .
Sending build context to Docker daemon  6.037MB
Step 1/3 : FROM scratch
 ---> 
Step 2/3 : ADD target/x86_64-alpine-linux-musl/release/hello-server /hello-server
 ---> Using cache
 ---> f480ad66fe7c
Step 3/3 : CMD ["/hello-server"]
 ---> Using cache
 ---> fb515237f3a8
Successfully built fb515237f3a8
Successfully tagged hello-server:rust
$ docker image ls
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
hello-server        rust                fb515237f3a8        9 minutes ago       6.03MB
rust                musl                1bead3b504b3        18 minutes ago      430MB
```
### Run the server
```bash
$ docker run -it -p 8080:8080 hello-server:rust
```
In another terminal, run the client
```bash
$ target/x86_64-alpine-linux-musl/release/hello-client
message: "Hello World"
```
