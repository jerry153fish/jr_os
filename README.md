# Toy OS for learning rust 

A Toy OS in rust

# How to start

1. install rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. put cargo bin into PATH

`export PATH=$PATH:~/.cargo/bin`

3. `cargo install bootimage`

4. `git clone git@github.com:jerry153fish/jr_os.git`

5. `cargo run`

NB: this will install the nightly rustc

need to run `rustc component add rust-src` and `rustc component add llvm-tools-preview` after nightly rustc be installed


# Test

`cargo test`
