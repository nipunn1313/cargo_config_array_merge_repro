fn main() -> anyhow::Result<()> {
    let config = cargo_config2::Config::load()?;
    let target = "aarch64-apple-darwin";
    let rustflags = config.rustflags(target)?;
    println!("Hello, world!");
    println!("{rustflags:?}");
    Ok(())
}
