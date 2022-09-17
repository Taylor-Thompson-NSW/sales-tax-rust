#![allow(unused)]

use std::env;
use std::io::{stderr, stdin, stdout, Write};

///This is basically tutorial style code on my part so far to familiarize myself
///with Rust. So this will almost be entirely deleted or refactored into the sales
///tax application soon.

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
    return *stream;
}

//TODO: Still just messing with these types.
fn get_dimension(input: &mut f32) -> f32 {
    let mut input_string = String::new();
    stdout().flush().expect("Failed to flush");
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

    println!("please input the sales information below");
    println!("----------");

    print!("shopify: ");
    get_dimension(&mut non_exempt_sales.shopify);
    println!("You wrote: {}", non_exempt_sales.shopify);
    println!("----------");

    print!("square:");
    get_dimension(&mut non_exempt_sales.square);
    println!("You wrote: {}", non_exempt_sales.square);
    println!("----------");

    print!("faire:");
    get_dimension(&mut exempt_sales.faire);
    println!("You wrote: {}", exempt_sales.faire);
    println!("----------");

    println!("Sales Tax TEST: {}", non_exempt_sales.shopify);
    println!("----------");

    math_for_non_exempt(&mut non_exempt_sales.shopify);
    println!("shopify math TEST: {}", non_exempt_sales.shopify);
    println!("----------");

    math_for_non_exempt(&mut non_exempt_sales.square);
    println!("square TEST: {}", non_exempt_sales.square);
    println!("----------");
}
