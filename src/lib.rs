use args::*;

pub mod args;

pub fn run(args: Arguments) {
    // Output arguments.
    println!("{:#?}", args);
}