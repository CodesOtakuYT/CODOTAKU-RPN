extern crate editline;

use f64;
use std::collections::HashMap;
use std::f64::consts::{E, PI};

const CMDS: [&'static str; 10] = [
    "log", "cos", "sin", "tan", "sqrt", "abs", "PI", "EPSILON", "INFINITY", "E",
];

fn list_possib(word: &str) -> Vec<&str> {
    let mut matches = Vec::<&str>::new();

    for cmd in &CMDS {
        if cmd.starts_with(word) {
            matches.push(cmd)
        }
    }

    matches
}

fn complete(word: &str) -> Option<&str> {
    let possib = list_possib(word);

    match possib.len() {
        1 => Some(&possib[0][word.len()..]),
        _ => None,
    }
}

extern "C" fn do_exit() -> editline::Status {
    return editline::Status::EOF;
}

const HISTORY_FILENAME: &str = "/tmp/codotaku.history";

fn main() {
    println!(
        "CODOTAKU RPN 1.0.0\nPress [Enter] or [Ctrl + X] without typing anything to exit properly."
    );

    let mut stack = Vec::new();
    let mut variables = HashMap::new();

    editline::set_list_possib(list_possib);
    editline::set_complete(complete);
    editline::read_history(HISTORY_FILENAME);
    editline::bind_key(editline::Key::Ctrl('x'), do_exit);

    loop {
        let input = editline::readline("> ").unwrap_or("");

        if input.is_empty() {
            break;
        }

        for token in input.split_whitespace() {
            match token {
                "+" => {
                    let value0: f64 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1 + value0;
                    stack.push(result);
                    println!("\t\t{value1} + {value0} = {result}");
                }
                "-" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1 - value0;
                    stack.push(result);
                    println!("\t\t{value1} - {value0} = {result}");
                }
                "*" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1 * value0;
                    stack.push(result);
                    println!("\t\t{value1} * {value0} = {result}");
                }
                "/" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1 / value0;
                    stack.push(result);
                    println!("\t\t{value1} / {value0} = {result}");
                }
                "%" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1 % value0;
                    stack.push(result);
                    println!("\t\t{value1} % {value0} = {result}");
                }
                "^" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1.powf(value0);
                    stack.push(result);
                    println!("\t\t{value1} ^ {value0} = {result}");
                }
                "log" => {
                    let value0 = stack.pop().expect("'+' Requires 2 additional operands");
                    let value1 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value1.log(value0);
                    stack.push(result);
                    println!("\t\tlog({value1}, base = {value0}) = {result}");
                }
                "sqrt" => {
                    let value: f64 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value.sqrt();
                    stack.push(result);
                    println!("\t\tsqrt({value}) = {result}");
                }
                "cos" => {
                    let value: f64 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value.cos();
                    stack.push(result);
                    println!("\t\tcos({value}) = {result}");
                }
                "sin" => {
                    let value: f64 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value.sin();
                    stack.push(result);
                    println!("\t\tsin({value}) = {result}");
                }
                "tan" => {
                    let value: f64 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value.tan();
                    stack.push(result);
                    println!("\t\ttan({value}) = {result}");
                }
                "abs" => {
                    let value: f64 = stack.pop().expect("'+' Requires 1 additional operand");
                    let result = value.abs();
                    stack.push(result);
                    println!("\t\tabs({value}) = {result}");
                }
                "PI" => {
                    stack.push(PI);
                }
                "E" => {
                    stack.push(E);
                }
                "EPSILON" => {
                    stack.push(f64::EPSILON);
                }
                "INFINITY" => stack.push(f64::INFINITY),
                _ => {
                    let value = token.parse::<f64>();
                    stack.push(match value {
                        Ok(x) => x,
                        Err(_) => {
                            let v = variables.get(token);
                            match v {
                                Some(x) => *x,
                                None => {
                                    let prompt = format!("\t{}? ", &token);
                                    let input = editline::readline(&prompt).unwrap();
                                    let result = input
                                        .parse::<f64>()
                                        .expect(&format!("Expected a number, got {token}"));
                                    variables.insert(token, result);
                                    result
                                }
                            }
                        }
                    });
                }
            }
        }

        println!("\t= {}", stack.pop().expect("Expected an output, got None"));
        if stack.len() != 0 {
            panic!("Unconsumed values, stack is not empty!");
        }

        variables.clear();
    }

    editline::write_history(HISTORY_FILENAME);
}
