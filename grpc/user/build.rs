// Cargo will pick up on the existence of this file, then compile and execute it before the rest of the crate is built. This can be used to generate code at compile time. 

fn main() {
    tonic_build::compile_protos("proto/user/user.proto").unwrap();
}
