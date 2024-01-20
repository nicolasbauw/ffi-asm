# Using FFI to Call Assembly Functions from Rust

This is a mini-howto for people wanting to call assembly functions from their Rust code.
- assembly is not inline assembly, but a separate .s source file
- just replace mnemonics to fit your target CPU (here, two examples for armv6 and aarch64)

Cross-compilation for Raspberry Pi 1B:  
```
rustup target add arm-unknown-linux-musleabihf  
brew install arm-linux-gnueabihf-binutils  
cargo build --target="arm-unknown-linux-musleabihf" --release
```

Linux users could replace musleabihf by gnueabihf.