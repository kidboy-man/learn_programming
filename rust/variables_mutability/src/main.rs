use std::io; // we need this library to do operation with input or output in terminal

const PHI: f64 = 3.14;

fn main() {
    let x = 5; 
    // this is immutable, can only be assigned once
    // means that we do not allow to change the value of x
    println!("The value of x is: {}", x);

    // now lets define a new mutable variable
    let mut y = 8;
    println!("The value of y is: {}", y);

    // since it is mutable. we can change its value
    y = 9;
    println!("The value of y is: {}", y);

    // using constants
    // constants are immuatable by defalut and be declared using keyword 'const'
    // although constants and immutable variables seem similar, they have significant differences
    // a constant can be declared at any scope (look at line 1)
    // type of a constant must be explicitly defined

    println!("the value of Phi is: {}", PHI);

    let mut input_string = String::new();
    println!("input the circle's radius in float64 format");
    io::stdin()
        .read_line(&mut input_string) // read input from terminal
        .expect("Failed to read input"); // throw an error, if occur
    
    let radius = input_string.trim();
        match radius.parse::<f64>() {
            Ok(f) => 
                println!("your cirlce's radius input: {}\n area of the cirlce: {}\n circumference of the cirlce: {}", 
                f, area_of_circle(f), circumference_of_circle(f)),
            Err(..) => println!("this was not a float64: {}", radius),
        };

    // shadowing
    // it is analogous to the concept of mutable variables
    // but instead of reassigning the value, we redeclaring the variable
    let z = 5;
    let z = x + 1;
    let z = x * 2;
    println!("The value of z is: {}", z);

    // since we redeclare the variable, we able to assign different types into it
    let z = "now z is a string";
    println!("The value of z is: {}", z)

}

fn area_of_circle(radius: f64) -> f64 {
    return PHI * radius.powf(2.0)
}

fn circumference_of_circle(radius: f64) -> f64 {
    return 2.0 * PHI * radius
}