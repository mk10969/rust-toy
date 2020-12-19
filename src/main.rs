mod test_ctrl_c;
mod test_duct;
mod test_maplit;

use anyhow::Context as _;
use std::fs;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");

    if let Err(e) = main_comandline() {
        println!("{:?}", e);
    }
}

fn main_anyhow() -> anyhow::Result<()> {
    // カラーリングは、してくれない。。。
    // "Error: "や"Cuased by:"等・・・
    fs::read_to_string("./nothing.txt").with_context(|| "Could not read...")?; // Errorの場合のコメント
    Ok(())
}

fn main_comandline() -> anyhow::Result<()> {
    Ops::from_args();
    Ok(())
    // $ ./target/release/rust-toy --help
    // Hello, world!
    // rust-toy 0.1.0

    // USAGE:
    //     rust-toy

    // FLAGS:
    //     -h, --help       Prints help information
    //     -V, --version    Prints version information
}

#[derive(StructOpt)]
struct Ops {}

#[test]
fn test_main() {
    println!("Hello, world!");
}
