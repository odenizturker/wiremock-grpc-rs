#[allow(unused_imports, dead_code)]
mod codegen_test;
#[allow(unused_imports)]
mod features_test;
#[allow(unused_imports)]
mod mocks_test;
#[allow(unused_imports)]
mod validations_test;

pub mod greeter_proto {
    tonic::include_proto!("hello");
}