
# Building for linux

from: http://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html

    rustup target add x86_64-unknown-linux-musl
    brew install FiloSottile/musl-cross/musl-cross
    cargo build --release --target=x86_64-unknown-linux-musl
