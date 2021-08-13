extern crate clap;
extern crate louis;
extern crate pretty_env_logger;

use clap::{App, Arg};
use louis::Louis;
use std::io::{self, BufRead, Write};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("lou_translate_rs")
        .version("0.1")
        .about("A clone of lou_translate made using the Rust bindings")
        .arg(
            Arg::with_name("TABLE")
                .help("The translation table(s) to use")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("forward")
                .short("-f")
                .long("--forward")
                .help("Forward translation using the given table (on by default)"),
        )
        .arg(
            Arg::with_name("backward")
                .short("-b")
                .long("--backward")
                .help("Backward translation using the given table")
                .conflicts_with("forward"),
        )
        .get_matches();

    let louis = Louis::new().unwrap();
    let table = matches
        .values_of("TABLE")
        .unwrap()
        .collect::<Vec<_>>()
        .join(",");
    let stdin = io::stdin();
    let ilock = stdin.lock();
    let stdout = io::stdout();
    let mut olock = stdout.lock();
    for line in ilock.lines() {
        writeln!(olock, "{}", louis.translate_simple(
            &table, 
            &line.unwrap(), 
            matches.is_present("backward"), 
            0)
        ).unwrap();
    }
}
