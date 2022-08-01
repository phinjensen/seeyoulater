Steps I had to take to be able to build for Windows x86\_64 from Linux:

1. Add toolchain: `rustup target add x86_64-pc-windows-gnu`
2. Install GCC multilib: `sudo apt install gcc-multilib-x86-64-linux-gnux32`
3. Install mingw-w64: `sudo apt install mingw-w64`
