
#[ctor::ctor]
fn ctor() {
    println!("Hello from Rust!");
    std::process::exit(112)
}