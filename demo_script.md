# install rustup

## setup
```
asciinema rec 01-install-rustup.json
docker run --rm --name hello_rust -it booyaa/hello_rust:base_build
clear
```

```
rustup # proof it’s not installed yet
curl https://sh.rustup.rs -sSf | sh # let's install rustup
source $HOME/.cargo/env # only need to do this here, future shells will be initialised
export USER=ferris # ignore this, my dockerese is poor

clear
rustup -V && cargo -V && rustc -V # version check 

clear
cargo new --bin hello # lets validate our installation
cd hello
cat src/main.rs
cargo run
cd ..

clear
rustup show 
# first line is the target is x86_64-unknown-linux-gnu (macos would be x86_64-apple-darwin)
# second ine tells you which release channel you using (stable)
# third is the version of the compler

rustup install nightly
rustup show
# note which channel is the default (stable)

cargo new --bin nightly_project
cd nightly_project
rustup override set nightly
rustup show # note the last line, we're going to use the nightly compiler
```

exit demo stop asciinema record

# Toolchain