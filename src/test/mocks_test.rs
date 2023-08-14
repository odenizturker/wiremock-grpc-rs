use tonic::Code;

use crate::{test::mocks_test::wiremock_gen::MyMockServer, wiremock::builder::{MockBuilder, Then}, hello::HelloReply};

mod wiremock_gen {
    crate::generate!("hello.Greeter", MyMockServer);
}

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
