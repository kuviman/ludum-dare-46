fn main() {
    println!("cargo:rerun-if-env-changed=LD46_SERVER_HOST");
    println!("cargo:rerun-if-env-changed=LD46_SERVER_PORT");
    println!("cargo:rerun-if-env-changed=LD46_CONNECT");
}
