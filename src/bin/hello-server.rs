use std::thread;

use rust_grpc_hello::hellosvc_grpc::*;
use rust_grpc_hello::hellosvc::*;

struct HelloSvcImpl;

impl HelloSvc for HelloSvcImpl {
    fn hello(&self,
    _m: grpc::RequestOptions,
    req: HelloRequest,)
    -> grpc::SingleResponse<HelloResponse> {
        let mut res = HelloResponse::new();

        println!("received hello request from {}", 
            req.get_name()
        );

        let message = "Hello ".to_string() + &req.get_name();
        res.set_message(message);
        grpc::SingleResponse::completed(res)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(HelloSvcServer::new_service_def(HelloSvcImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }    
}