//- module root
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)] //- outer attribute
#[command(author, version, about, long_about = None)] //- outer attributes
struct Args {
    /// Name of the person to greet
    #[arg(short, long)] //- inner attribute
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)] //- inner attribute
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

}