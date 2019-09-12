// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait HelloSvc {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::hellosvc::HelloRequest) -> ::grpc::SingleResponse<super::hellosvc::HelloResponse>;
}

// client

pub struct HelloSvcClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_hello: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::hellosvc::HelloRequest, super::hellosvc::HelloResponse>>,
}

impl ::grpc::ClientStub for HelloSvcClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HelloSvcClient {
            grpc_client: grpc_client,
            method_hello: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hello.HelloSvc/hello".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl HelloSvc for HelloSvcClient {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::hellosvc::HelloRequest) -> ::grpc::SingleResponse<super::hellosvc::HelloResponse> {
        self.grpc_client.call_unary(o, p, self.method_hello.clone())
    }
}

// server

pub struct HelloSvcServer;


impl HelloSvcServer {
    pub fn new_service_def<H : HelloSvc + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/hello.HelloSvc",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hello.HelloSvc/hello".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.hello(o, p))
                    },
                ),
            ],
        )
    }
}
