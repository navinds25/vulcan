extern crate tonic_build;

fn main() {
    match tonic_build::compile_protos("../proto/service.proto") {
        Ok(n) => println!("success: {:?}", n),
        Err(err) => println!("error: {}", err),
    }
}