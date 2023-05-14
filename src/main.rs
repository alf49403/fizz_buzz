use std::io;

fn main() {
    println!("How far to go?");
    let stop_at = get_u32();
    for x in 1..=stop_at {
        println!("{}", fizz_it(x));
    }
}

fn get_u32() -> u32 {
    let mut number = String::new();
    loop {
        if io::stdin().read_line(&mut number).is_ok() {
            match number.trim().parse::<u32>() {
                Ok(x) => return x,
                Err(_) => {
                    println!("Put a natural number in");
                    number.clear();
                },
            }
        }
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
