FROM scratch
ADD target/x86_64-alpine-linux-musl/release/hello-server /hello-server
EXPOSE 8080
CMD ["/hello-server"]