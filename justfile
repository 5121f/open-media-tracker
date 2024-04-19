install: build_release
    cp target/release/zcinema ~/.local/bin

build_release:
    cargo build --release
