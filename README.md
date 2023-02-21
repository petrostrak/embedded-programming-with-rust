## Embedded programming with Rust

### For cross-compiling (it only needs to run once)
```
rustup target add thumbv7em-none-eabihf
```

### itmdump
```
cargo install itm
```
Verify the version is >=0.3.1
```
$ itmdump -V
itmdump 0.3.1
```
### cargo-binutils

Install llvm-tools-preview
```
rustup component add llvm-tools-preview
```
Install cargo-binutils
```
cargo install cargo-binutils
```