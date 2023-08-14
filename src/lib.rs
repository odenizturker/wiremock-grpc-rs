pub mod wiremock;
pub mod test;

pub mod hello {
    tonic::include_proto!("hello");
}
