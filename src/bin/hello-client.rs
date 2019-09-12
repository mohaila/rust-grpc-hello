use rust_grpc_hello::hellosvc_grpc::*;
use rust_grpc_hello::hellosvc::*;
use grpc::ClientStubExt;


fn main() {
    let client = HelloSvcClient::new_plain("127.0.0.1", 8080, Default::default()).unwrap();

    let mut req = HelloRequest::new();
    req.set_name("World".to_string());

    let resp = client.hello(grpc::RequestOptions::new(), req);
    match resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }
}
