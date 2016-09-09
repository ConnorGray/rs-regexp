extern crate regexp;

fn main() {
    // use std::io;
    // let mut input = String::new();
    // io::stdin().read_line(&mut input)
    //     .expect("Failed to read from stdin");
    // let input = input.trim();

    // let input = "(b|cd++)";
    let input = "(a|b(q|d|(sd)+)a?)";

    regexp_data_dump(input);
}

/// (+

fn regexp_data_dump(regexp_string: &str) {
    println!("------ DATA DUMP FOR REGEX {} ------", regexp_string);

    let regexp = match regexp::regexp_from_string(&regexp_string) {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to parse regexp with error: {}", err);
            return;
        }
    };

    println!("{:?}", regexp);
    regexp::print_regexp(&regexp);
    println!("Input:\t\t{}", regexp_string);
    println!("Reconstructed:\t{}", regexp::regexp_to_string(&regexp));

    println!("------ END DATA DUMP ------");
}
