fn main() {
    // easy assignment
    // q1
    // Declare an immutable variable x with the value 10.
    let x: i32 = 10;
    println!("x {x}");
    // Declare a mutable variable y with the value 5 and then change its value to 20.
    let mut y: i32 = 5;
    println!("y {y}");
    y = 20;
    println!("y {y}");
    // Create a constant named MAX_POINTS with the value 100,000.
    const MAX_POINTS: i32 = 100_000;
    println!("MAX_POINTS {MAX_POINTS}");

    // Q2
    // Declare a variable of each of the following types: i32, f64, bool, and char
    let integer: i32 = 32;
    let float: f64 = 64.23;
    let booliya: bool = true;
    let character: char = 'a';
    println!("integer {integer} , float {float},booliya {booliya},character {character}");
    // Create an array with the values 1, 2, 3, 4, 5.
    let array_int: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_init : {:?}", array_int);
    // Create a tuple with three different data types.
    // let tuple = (23,"Vedar",true)
    let tuple: (i32, &'static str, bool) = (23, "Vedar", true);
    println!("tuple {:?}", tuple);

    hello_ji();
    print_integer(707);
    let a: i32 = 707;
    let b: i32 = 708;
    let result: i32 = sumation(a, b);
    println!("sum of {} and {} is {}", a, b, result);
    // Write an if statement that checks if a number is positive.
    if a > 0 {
        println!("{} is greater than zero so it is posative", a);
    }
    if b % 2 == 0 {
        println!("{} is even", b);
    } else {
        println!("{} is odd", b);
    }
    // Write a loop that prints the numbers from 1 to 5.
    println!("using for loop");
    for i in 0..6 {
        println!("i = {}", i);
    }
    println!("1 - 5 using while loop");
    let mut i:i16 = 0;
    while i < 6 {
        println!("i = {}", i);
        i+=1;
    }
    while i > 5 {
        if i== 8{
            break;
        }
        println!("i = {}", i);
        i+=1;
    }
    i = 700;
    while i < 710 {
        i+=1;
        if i== 708{
            continue;
        }
        println!("i = {}", i);
    }
}

// Q3
// Write a function that takes no arguments and prints "Hello, world!".
fn hello_ji() {
    print!("Hello , World");
}

fn print_integer(x: i32) {
    println!("The given number {}", x);
}
/*
function to add two number and return i32 as resutlt
*/
fn sumation(x: i32, y: i32) -> i32 {
    x + y
}
