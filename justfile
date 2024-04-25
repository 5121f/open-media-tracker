install: build_release
    cp target/release/zcinema ~/.local/bin

build_release:
    cargo build --release

build_for_windows:
    cargo build --release --target x86_64-pc-windows-gnu
