These are the dockerfile (docker image templates) for the Rust talk. You shouldn't really use this for day to day use. There's an official `rust` image which can be got via `docker pull rust`.

These images were created to fit the user requirement of Ubuntu 16.04 (Xenial), where as the official image uses Debian.

# Usage

`docker run -it booyaa/hello_rust:base_build` - this will give you a base build before rust has been installed
`docker run -it booyaa/hello_rust:rustup` - this will give you a build with rustup installed with the default stable toolchain

# Build

`docker build -t hello_rust:base_build .`


