use std::{collections::HashMap, io, process, str::Split};

use algorithms_and_data_structure::BooleanExpression;

fn main() {
    let mut buffer = String::new();

    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        match create_boolean_expression(&buffer).evaluate() {
            Ok(r) => println!("{}", r),
            Err(e) => {
                eprintln!("Problem evaluating boolean expression: {}", e);
                process::exit(1);
            }
        }

        buffer.clear();
    }
}

fn create_boolean_expression(buffer: &str) -> BooleanExpression {
    let mut buffer_splitted = buffer.trim().split(' ');
    let identifier_table = create_identifier_table(&mut buffer_splitted);

    match BooleanExpression::build(buffer_splitted.collect(), identifier_table) {
        Ok(be) => be,
        Err(e) => {
            eprintln!("Problem creating boolean expression: {}", e);
            process::exit(1);
        }
    }
}

fn create_identifier_table(buffer: &mut Split<char>) -> HashMap<String, bool> {
    let qtt_input = read_quantity_inputs(buffer);

    if qtt_input < 1 {
        eprintln!("The boolean expression must have at least 1 identifier");
        process::exit(1);
    }

    let mut result = HashMap::new();

    for n in 1..=qtt_input {
        match buffer.next() {
            Some(identifier_text) => {
                let mut identifier_splitted = identifier_text.split('=');
                let name = read_identifier_name(&mut identifier_splitted, n);
                let value = read_identifier_value(&mut identifier_splitted, n);

                result.insert(name, value);
            }
            None => {
                eprintln!("Missing the {}nth identifier", n);
                process::exit(1);
            }
        }
    }

    result
}

fn read_quantity_inputs(buffer: &mut Split<char>) -> u8 {
    match buffer.next() {
        Some(qtt_inputs_text) => match qtt_inputs_text.parse::<u8>() {
            Ok(qtt_input) => qtt_input,
            Err(_) => {
                eprintln!(
                    "Expected the number of input values to read, received \"{}\" instead",
                    qtt_inputs_text
                );
                process::exit(1);
            }
        },
        None => {
            eprintln!("Missing the number of input values to read");
            process::exit(1);
        }
    }
}

fn read_identifier_name(buffer: &mut Split<char>, nth: u8) -> String {
    if let Some(name) = buffer.next() {
        if name.is_empty() {
            eprintln!("Missing the name of the {}nth identifier", nth);
            process::exit(1);
        }

        return name.to_owned();
    } else {
        eprintln!("Missing the name of the {}nth identifier", nth);
        process::exit(1);
    }
}

fn read_identifier_value(buffer: &mut Split<char>, nth: u8) -> bool {
    match buffer.next() {
        Some(value) => match value.parse::<u8>() {
            Ok(value) => value > 0,
            Err(_) => {
                eprintln!("Expected the value of the {}nth identifier", nth);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Missing the value of the {}nth identifier", nth);
            process::exit(1);
        }
    }
}
