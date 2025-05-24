#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
// Define an enum Direction with values: Up, Down, Left, and Right.
// Write a function move_player that takes a Direction and prints the direction.
// Use match to handle each case.
enum Direction{
    Up,
    Down,
    Left,
    Right
}
fn move_player(direction:&Direction){
    match direction{
        Direction::Up => println!("Move up"),
        Direction::Down => println!("Move Down"),
        Direction::Left => println!("Move Left"),
        Direction::Right => println!("Move right"),
    }
}

// Define an enum Coin with:
// Penny
// Nickel
// Dime
// Quarter(String) – holds the U.S. state name.
// Write a function coin_value that returns the value of the coin and prints the state for quarters.


#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}
fn coin_value(coin:&Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State of quareter from : {}",state);
            25
        }
    }
}
// Write a function next_even that:
// Takes Option<i32>
// Returns Some(x + 1) if x is even
// Returns None if x is odd or None

fn next_even(x:Option<i32>) -> Option<i32>{
    match x{
        Some(x) if x % 2 == 0 => Some(x +1),
        _ => None
    }
}

fn main() {
    
    let dir1 = Direction::Up;
    let dir2 = Direction::Down;
    let dir3 = Direction::Left;
    let dir4 = Direction::Right;
    move_player(&dir1);
    // move_player(dir1);
    move_player(&dir2);
    move_player(&dir3);
    move_player(&dir4);
    // move_player(&dir4);

// Define an enum IpAddr with:
// V4(String)
// V6(String)
// Create and print two instances: one with IPv4 address, another with IPv6.
    let ipv4 = IpAddr::V4(String::from("192.168.2.233"));
    let ipv6 = IpAddr::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("ipv4 {:#?}",ipv4);
    println!("ipv6 {:#?}",ipv6);
    // Coin instance
    let coin1 = Coin::Quarter(String::from("Janakpur"));
    println!("Value of {:?} is {} ",coin1,coin_value(&coin1));

    let val1 = Some(4);
    let val2 = Some(7);
    let val3 = None;

    println!("Next even for {:?} is {:?}", val1, next_even(val1)); // Some(5)
    println!("Next even for {:?} is {:?}", val2, next_even(val2)); // None
    println!("Next even for {:?} is {:?}", val3, next_even(val3)); // None

    let some_username = Some(String::from("mukesh123"));
    if let Some(username) = some_username{
        println!("Username is {}",username);
    }
is_prime(37);
}

fn is_prime(num:i32) -> bool{
    let i =1;
    for i in 2..10 {
        println!(" i {}",i);
        if num % i == 0{
            return false
        }
    }
    true
}