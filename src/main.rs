mod args;

use std::process;

fn main() {
    // Parse arguments.
    let args = args::parse();
    if args.show_help {
        args::print();
        process::exit(0);
    }

    // Output arguments.
    println!("{:#?}", args);
}
