use clap::Parser;

/// A Rust implementation of "Fizz Buzz"
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// How many numbers to show the "Fizz Buzz" output for
    #[arg(short, default_value_t = 15)]
    n: u32
}

fn main() {
    let args = Args::parse();
    for x in 1..=args.n {
        if let Some(s) = fizz_it(x) {
            println!("{s}");
        } else {
            println!("{x}");
        }
    }
}

fn fizz_it(x: u32) -> Option<&'static str> {
    let mod5 = x % 5 == 0;
    if x % 3 == 0 {
        if mod5 {
            Some("fizz buzz")
        } else {
            Some("fizz")
        }
    } else if mod5 {
        Some("buzz")
    } else {
        None
    }
}
