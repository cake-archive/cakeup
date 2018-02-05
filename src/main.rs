mod args;

fn main() {
    // Parse arguments.
    let args = args::parse();

    // Output arguments.
    println!("{:#?}", args);
}
