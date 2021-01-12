mod test_ctrl_c;
mod test_duct;
mod test_maplit;

use anyhow::Context as _;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, path::PathBuf};
use structopt::{clap, clap::arg_enum, StructOpt};

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
    Opt::from_args();
    Ok(())

    // #[derive(StructOpt)]
    // struct Ops {}
    // デフォルトの場合下記が出力される。

    // $ ./target/release/rust-toy --help
    // Hello, world!
    // rust-toy 0.1.0

    // USAGE:
    //     rust-toy

    // FLAGS:
    //     -h, --help       Prints help information
    //     -V, --version    Prints version information
}

lazy_static! {
    static ref THREADS: String = format!("{}", num_cpus::get());
}
#[derive(Debug, StructOpt)]
#[structopt(name = "App name")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(name = "WORDS")]
    pub words: Vec<String>,

    #[structopt(short = "n")]
    pub count: Option<usize>,

    #[structopt(short = "p", long = "path")]
    pub path: PathBuf,

    #[structopt(short = "r", long = "regex")]
    pub regex: Regex,

    #[structopt(
        short = "t",
        long = "threads",
        default_value(&THREADS),
        value_name = "NUM"
    )]
    pub threads: usize,

    #[structopt(
        short = "m",
        long = "method",
        possible_values(&Method::variants())
    )]
    pub method: Option<Method>,

    #[structopt(short = "a", conflicts_with_all(&["b", "c"]))]
    pub a: bool,

    #[structopt(short = "b", conflicts_with_all(&["a", "c"]))]
    pub b: bool,

    #[structopt(short = "c", conflicts_with_all(&["a", "b"]))]
    pub c: bool,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(Debug, StructOpt)]
pub enum Sub {
    #[structopt(name = "sub1", about = "sub command1")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Sub1,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Method {
        A,
        B,
        C,
    }
}
