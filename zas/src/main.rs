use clap::{load_yaml, App};
use colored::*;

mod dictionary;
mod file;
use file::read_file;
mod compiler;
use compiler::Compiler;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        println!("BUILDING");
        let input_fname = matches.value_of("input").unwrap().to_string();
        let output_fname = matches.value_of("output").unwrap().to_string();
        let verbosity = matches.is_present("verbose");

        let mut compiler = Compiler::new(read_file(input_fname));
        compiler.compile();
    } else {
        eprintln!("{}: wasn't provided {}\nFor help run `{}`", "ERROR".bright_red(), "SUBCOMMAND".cyan(), "zas --help".green());
        std::process::exit(1);
    }
}
