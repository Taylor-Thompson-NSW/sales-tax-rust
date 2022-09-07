#![allow(unused)]

use std::io::stdin;

use std::env;

fn get_dimension(input: &mut String) {
    stdin().read_line(input).ok().expect("Failed to read line");
}

fn calculate_area(length: &String, width: &String) -> i32 {
    let length_to_int = length.trim().parse::<i32>().unwrap();
    let width_to_int = width.trim().parse::<i32>().unwrap();
    length_to_int * width_to_int
}

fn main() {
    println!("Write something as a test");
    let mut input_string = String::new();
    get_dimension(&mut input_string);
    println!("You wrote: {}", input_string);

    println!("What is the width?");
    let mut input_width = String::new();
    get_dimension(&mut input_width);
    println!("You wrote: {}", input_width);

    println!("What is the length?");
    let mut input_length = String::new();
    get_dimension(&mut input_length);
    println!("You wrote: {}", input_length);

    let area = calculate_area(&input_length, &input_width);

    println!(
        "The dimensions you provided are {L} and {W}",
        L = input_length.trim(),
        W = input_width
    );

    println!("Area of your shape is {}", area);
}
