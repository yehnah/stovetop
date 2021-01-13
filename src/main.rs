// use stovetop;
use clap::{App, Arg};

fn main() {
    // Gather values from CLi arguments.
    let matches = App::new("Stovetop")
       .version("0.0.1")
       .about("Simple file templating system written in Rust")
       .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
        .arg(Arg::with_name("TEMPLATE")
                .help("Sets the template file to use")
                .required(true)
                .index(2))
        .arg(Arg::with_name("OUTPUT")
                .help("Sets the directory to output")
                .required(true)
                .index(3))
       .get_matches();

    // Generate template directory based on CLi argument values.
    stovetop::generate(
        matches.value_of("INPUT").unwrap(),
        matches.value_of("TEMPLATE").unwrap(),
        matches.value_of("OUTPUT").unwrap(),
        None
    ).unwrap();
}
