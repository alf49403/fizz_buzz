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
        if let Ok(_) = io::stdin().read_line(&mut number) {
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
    let mut output_vec: Vec<String> = vec![];
    if x % 3 == 0 {
        output_vec.insert(0, "fizz".to_string());
    }
    if x % 5 == 0 {
        output_vec.insert(0, "buzz".to_string());
    }
    if output_vec.len() == 0 {
        output_vec.insert(0, x.to_string());
    }
    let output_text = output_vec.join(" ");
    output_text
}
