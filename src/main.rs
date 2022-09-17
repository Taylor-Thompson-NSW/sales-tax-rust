#![allow(unused)]

use std::io::stdin;

use std::env;

///This is basically tutorial style code on my part so far to familiarize myself
///with Rust. So this will almost be entirely deleted or refactored into the sales
///tax application soon.

//TODO: Get to it.

///////////////////////////

// fn get_dimension(input: &mut String) {
//     stdin().read_line(input).ok().expect("Failed to read line");
// }

// fn calculate_area(length: &String, width: &String) -> i32 {
//     let length_to_int = length.trim().parse::<i32>().unwrap();
//     let width_to_int = width.trim().parse::<i32>().unwrap();
//     length_to_int * width_to_int
// }

// fn main() {
//     println!("Write something as a test");
//     let mut input_string = String::new();
//     get_dimension(&mut input_string);
//     println!("You wrote: {}", input_string);

//     println!("What is the width?");
//     let mut input_width = String::new();
//     get_dimension(&mut input_width);
//     println!("You wrote: {}", input_width);

//     println!("What is the length?");
//     let mut input_length = String::new();
//     get_dimension(&mut input_length);
//     println!("You wrote: {}", input_length);

//     let area = calculate_area(&input_length, &input_width);

//     println!(
//         "The dimensions you provided are {L} and {W}",
//         L = input_length.trim(),
//         W = input_width
//     );

//     println!("Area of your shape is {}", area);
// }
//
// /////////////////////
//
//

struct NonExemptSales {
    shopify: f32,
    square: f32,
}
struct ExemptSales {
    faire: f32,
    amazon: f32,
    paypal: f32,
    venmo: f32,
    quickbooks: f32,
    zelle: f32,
}

fn math_for_non_exempt(stream: &mut f32) -> f32 {
    let tax_rate: f32 = 0.07;

    let mut stream_math = *stream / tax_rate;
    *stream = stream_math;
    return *stream
}

//TODO: Still just messing with these types.
fn get_dimension(input: &mut f32) -> f32 {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    //Look more into Deref, Borrows, Types, and Parsing ... I know that's a lot.
    let input_int: f32 = input_string.trim().parse().unwrap();
    *input = input_int;
    return *input;
}

fn main() {
    let mut exempt_sales = ExemptSales {
        faire: 0.0,
        amazon: 0.0,
        paypal: 0.0,
        venmo: 0.0,
        quickbooks: 0.0,
        zelle: 0.0,
    };

    let mut non_exempt_sales = NonExemptSales {
        shopify: 0.0,
        square: 0.0,
    };

    // TODO: test with just a couple of channels, and figure the best way to  separate
    // exempt and non-exempt sales

    print!("Please input the sales information below:");
    println!("----------");

    println!("shopify:");
    get_dimension(&mut non_exempt_sales.shopify);
    println!("You wrote: {}", non_exempt_sales.shopify);

    // println!("square:");
    // get_dimension(&mut non_exempt_sales.square);
    // println!("You wrote: {}", non_exempt_sales.square);

    // println!("faire:");
    // get_dimension(&mut exempt_sales.faire);
    // println!("You wrote: {}", exempt_sales.faire);

    println!("Sales Tax TEST: {}", non_exempt_sales.shopify);

    math_for_non_exempt(&mut non_exempt_sales.shopify);
    println!("MATH TEST: {}", non_exempt_sales.shopify);
}
