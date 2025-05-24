fn main() {
    let s = String::from("hello"); // s comes into scope
    // s's value moves into the function and so is no longer valid here
    takes_ownership(s);
    let s2 = gives_ownership();
    println!("s2 call's value : {}", s2);

    let s3 = gives_take_ownership_mr_robinhood(s2);
    println!("Its s3: {}", s3);
    let (s4, len) = calcute_length();
    println!("Calculete Lenght ");
    println!("\n The length of '{}' \t  : {}",s4,len);
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it's okay to
    // still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("Ownership lele bhai, gives_ownership bale vaiya ne diya hai");
    some_string
}

fn gives_take_ownership_mr_robinhood(lootle: String) -> String {
    lootle
}

fn calcute_length() -> (String, usize) {
    let hisab_bale = String::from("Hello");
    let length = hisab_bale.len();
    (hisab_bale, length)
}
