use tonic::Code;

mod wiremock_gen {
    wiremock_grpc::generate!("hello.Greeter", MyMockServer);
}

use wiremock_gen::*;
use wiremock_grpc::*;

use crate::greeter_proto::HelloReply;

#[tokio::test]
#[should_panic(expected = "Server terminated with unmatched rules: \n/")]
async fn mock_builder() {
    let mut server = MyMockServer::start_default().await;

    server.setup(
        MockBuilder::when()
            .path("/")
            .then()
            .return_status(Code::AlreadyExists),
    );

    server.setup(
        MockBuilder::when()
            .path("/")
            .then()
            .return_status(Code::AlreadyExists)
            .return_body(|| HelloReply {
                message: "Hello".into(),
            }),
    );
}
