fn main() {
    // Declare a String variable inside a block (inner scope).
    {
        let str: String = String::from("VEDAR YODHA");
        print!("strung is {}", str);
        let stri2 = func_to_take_str(str);
        print!("stri2 is {}", stri2);
    }
    // What happens when the block ends?
    // when block ends ,the scope of the varible ends

    // Q2
    // Declare a String and assign it to another variable. Explain what happens to ownership.
    let string1 = String::from("LOL");
    let string2: String = string1;
    // Ans -> ownership of string"LOL" is transfered from string1 to string2 now string1 is no more pointing toward the value :"LOL"
    let store_result = str_len(string2);
    println!("Returned len and string  {:?}", store_result);
   println!("P2 Q1");
   let mut for_p1_q1 = String::from("GADDI : ");
//    take_str_ref(&mut for_p1_q1);
   take_str_ref(&mut for_p1_q1);
   println!("{}",for_p1_q1); //GADDI , Lamborgini

}

// Q1
// Create a function that takes a String as an argument.
fn func_to_take_str(name: String) -> String {
    name
}
// What happens when you pass a String variable to this function?
// ANs -> the ownership transfer to the entity inside the function where the variable is passed

// What happens when you return a String from this function?
// Ans -> Function give its string;s data ownership to the variable which is storing the returned value

// Q2
// Write a function that takes a String, calculates its length, and returns the String and the length as a tuple.
fn str_len(str: String) -> (usize, String) {
    (str.len(), str)
}

// part 2 Q1
// Write a function that takes a String as a reference and prints its length. Can the function modify the String?
fn take_str_ref(s:&mut String) {
    println!("Length of the passed string is {}",s.len());
    // s.push_str("Lamborgini");
    println!("{s}");
}
// we are able to modify the string