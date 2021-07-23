cd ../gemsExp-main
cargo clean
cargo build
cd ../bootloader-main
cargo clean
cargo builder --kernel-manifest ../gemsExp-main/Cargo.toml --kernel-binary ../gemsExp-main/target/debug/GEMS-EXPERIENCE
