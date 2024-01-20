# Using FFI to Call Assembly Functions from Rust

This is a mini-howto for people wanting to call assembly functions from their Rust code.
- assembly is not inline assembly, but a separate .s source file
- just replace mnemonics to fit your target CPU (here, aarch64)