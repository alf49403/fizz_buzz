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
        println!("{}", fizz_it(x));
    }
}

fn fizz_it(x: u32) -> String {
    let mut output_vec: Vec<&'static str> = vec![];
    if x % 3 == 0 {
        output_vec.push("fizz");
    }
    if x % 5 == 0 {
        output_vec.push("buzz");
    }

    if output_vec.is_empty() {
        x.to_string()
    } else {
        output_vec.join(" ")
    }
}
