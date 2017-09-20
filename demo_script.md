# Demo script

## TODO

```
# http://askubuntu.com/questions/493460/ddg#493467
sudo apt-add-repository ppa:zanchey/asciinema
sudo apt-get update
sudo apt-get install asciinema
```

## Setup

set default profile to `asciinema`, then new window and maximize.

# Install rustup

## Setup

```
asciinema rec 01-install-rustup.json
docker run --rm --name hello_rust -it booyaa/hello_rust:base_build
```

```
clear
rustup # proof that it has been not installed yet
curl https://sh.rustup.rs -sSf | sh # lets install rustup
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

```
asciinema rec 02-toolchains.json
docker run --rm --name hello_rust -it booyaa/hello_rust:rustup
source $HOME/.cargo/env && export USER=ferris && clear

```

```
rustup show 
# first line is the target is x86_64-unknown-linux-gnu (macos would be x86_64-apple-darwin)
# second line tells you which release channel you using (stable)
# third line is the version of the compler

rustup install nightly
rustup show
# note which channel is the default (stable)

cargo new --bin stable
cd stable
cat > src/main.rs <<EOF
fn main() {
    for i in 1..11 { 
        print!("{} ", i);
    }
    println!("");
    // output: 1 2 3 4 5 6 7 8 9 10
}
EOF
cargo run

cd ..
cargo new --bin nightly
cd nightly
cat > src/main.rs<<EOF
#![feature(inclusive_range_syntax)]

fn main() {
    for i in 1...10 { 
        print!("{} ", i);
    }
    println!("");
    // output: 1 2 3 4 5 6 7 8 9 10
}
EOF
rustup show
cargo run

rustup override set nightly
rustup show # note the last line, we're going to use the nightly compiler
```

## RLS

## Documentation