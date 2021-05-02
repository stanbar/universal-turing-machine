mod turing_machine;

use turing_machine::{Rule, Direction, TuringMachine};

use std::io::stdin;
use std::{thread, time};

fn main() {
    println!("Bits addition using Turing Machine");
    println!("Input two unsigned integers to add");

    let mut in1 = String::new();
    stdin().read_line(&mut in1).expect("Failed to read line");
    let first: u32 = in1.trim().parse().expect("Invalid first integer");

    let mut in2 = String::new();
    stdin().read_line(&mut in2).expect("Failed to read line");
    let second: u32 = in2.trim().parse().expect("Invalid second integer");

    let input_tape = format!("{:b}_{:b}", first, second);
    println!("Input tape is: {}", input_tape);

    let rules_bb5 = vec![
        // move right to the end of first block
        Rule::new("0", '0', '0', Direction::Right, "0"),
        Rule::new("0", '1', '1', Direction::Right, "0"),
        Rule::new("0", '_', '_', Direction::Right, "1"),
        // move right to the end of second block
        Rule::new("1", '0', '0', Direction::Right, "1"),
        Rule::new("1", '1', '1', Direction::Right, "1"),
        Rule::new("1", '_', '_', Direction::Left, "2"),
        // Subtract one in binary
        Rule::new("2", '0', '1', Direction::Left, "2"),
        Rule::new("2", '1', '0', Direction::Left, "3"),
        Rule::new("2", '_', '_', Direction::Right, "5"),
        // Move left to the end of first block
        Rule::new("3", '0', '0', Direction::Left, "3"),
        Rule::new("3", '1', '1', Direction::Left, "3"),
        Rule::new("3", '_', '_', Direction::Left, "4"),
        // Add
        Rule::new("4", '0', '1', Direction::Right, "0"),
        Rule::new("4", '1', '0', Direction::Left, "4"),
        Rule::new("4", '_', '1', Direction::Right, "0"),
        // Clean up
        Rule::new("5", '1', '_', Direction::Right, "5"),
        Rule::new("5", '_', '_', Direction::Stay, "halt"),
    ];
    let halting_states = vec!["halt"];
    let mut tm_summator = TuringMachine::new("0", halting_states, '_', rules_bb5, &input_tape);
    let mut steps = 0;
    while !tm_summator.is_done() {
        thread::sleep(time::Duration::from_millis(50));
        println!("{}", tm_summator);
        tm_summator.step();
        steps += 1;
    }
    println!("Steps: {}", steps);
    println!("Band lenght: {}", tm_summator.band.len());

    println!(
        "Output: {} + {} = {} = {:#b} + {:#b} = 0b{}",
        first,
        second,
        first + second,
        first,
        second,
        tm_summator.tape()
    );
}

