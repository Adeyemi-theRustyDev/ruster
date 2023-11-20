use std::io;

fn get_num_from_user(prompt: &str) -> u8 {
    let mut num_placeholder = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut num_placeholder)
        .expect("Couldn't readline");

    let num_placeholder: u8 = match num_placeholder.trim().parse() {
        Ok(num) => num,
        Err(_) => u8::MAX,
    };

    num_placeholder
}

fn main() {
    const MAX_INPUT: u8 = 20;
    let mut fib_series: Vec<u8> = vec![];
    let _no_of_terms = loop {
        let terms = get_num_from_user("No of terms: ");
        if terms != u8::MAX {
            break terms;
        }
    };

    let first_num = loop {
        let num = get_num_from_user("Enter first term (only nums from 1 to 20 are allowed): ");

        if num > u8::MIN && num <= MAX_INPUT {
            break num;
        }
    };

    fib_series.push(0);
    fib_series.push(1);
    fib_series.push(first_num);

    println!("Capacity: {}", fib_series.capacity());
}
