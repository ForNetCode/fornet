fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
    }

    let path = &["auth.proto", "config.proto"].map(|v| "../../protobuf/".to_owned() + v);
    tonic_build::configure().build_server(false).build_client(true)
        .compile(path, &["../../protobuf"])?;


    Ok(())
}