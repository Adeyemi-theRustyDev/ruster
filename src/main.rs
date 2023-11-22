use std::io;

fn get_num_from_user(prompt: &str) -> u16 {
    let mut num_placeholder = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut num_placeholder)
        .expect("Couldn't readline");

    let num_placeholder: u16 = match num_placeholder.trim().parse() {
        Ok(num) => num,
        Err(_) => u8::MAX.into(),
    };

    num_placeholder
}

fn main() {
    const MAX_INPUT: u16 = 20;
    let mut fib_series: Vec<u16> = Vec::new();
    let no_of_terms = loop {
        let terms = get_num_from_user("No of terms: ");
        if terms != u8::MAX.into() {
            break terms;
        }
    };

    let first_num = loop {
        let num = get_num_from_user("Enter first term (only nums from 1 to 20 are allowed): ");

        if num > u8::MIN.into() && num <= MAX_INPUT {
            break num;
        }
    };

    fib_series.push(0);
    fib_series.push(1);
    fib_series.push(first_num);

    
    while fib_series.len() != no_of_terms.into() {        
        let last_term = fib_series[fib_series.len() - 1];
        let pen_ultimate_term = fib_series[fib_series.len() - 2];
        let next_term = last_term + pen_ultimate_term;
        fib_series.push(next_term);
    }
    println!("Series: {:#?}", fib_series)    
}
