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
