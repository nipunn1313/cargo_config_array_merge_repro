Repro with
1) Add some rustflags to your ~/.cargo/config.toml eg
```
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/Users/nipunn/src/sold/build/ld64"]
```
2) `cargo run --verbose`

You see that cargo gives
```
     Running `/Users/nipunn/.rustup/toolchains/nightly-2024-03-20-aarch64-apple-darwin/bin/rustc --crate-name asdf --edition=2021 src/main.rs --error-format=json
  --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=102 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2
  -C split-debuginfo=unpacked -C metadata=06bcc3bf1e1fe4ca -C extra-filename=-06bcc3bf1e1fe4ca --out-dir /Users/nipunn/asdf/target/debug/deps
  -C incremental=/Users/nipunn/asdf/target/debug/incremental -L dependency=/Users/nipunn/asdf/target/debug/deps
  --extern anyhow=/Users/nipunn/asdf/target/debug/deps/libanyhow-05d11313be6d6150.rlib --extern cargo_config2=/Users/nipunn/asdf/target/debug/deps/libcargo_config2-d8752e4ee28a74a3.rlib
  -C link-arg=-fuse-ld=/Users/nipunn/src/sold/build/ld64 --cfg tokio_unstable`
```

However the output of cargo-config2 gives
```
Some(Flags { flags: ["--cfg", "tokio_unstable", "-C", "link-arg=-fuse-ld=/Users/nipunn/src/sold/build/ld64"] })
```
