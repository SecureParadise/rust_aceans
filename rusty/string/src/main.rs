fn main() {
    let _ax = "hello moto";
    // &str -->
    let greeting = String::from("Hello MoTo");
    println!("string is {}", greeting);
    // println!("{}",greeting.chars().nth(1000))
    let _char1 = greeting.chars().nth(0);

    // mehtod 1
    println!("method 1 --> efficient method ");
    for c in greeting.chars() {
        println!("{}", c);
    }
    //method2
    println!("method 2 --> alternative 1 method ");
    let chars: Vec<char> = greeting.chars().collect();
    println!("Character index 0: {}", chars[0]);
    println!("Character index 6: {}", chars[6]);
    //method 3
    println!("method 3 --> alternative 2 method ");
    let namaskaram = String::from("你好世界"); // "Hello World" in Chinese
    let bytes = namaskaram.as_bytes();
    println!("Byte at index 0: {}", bytes[0]); // Output: 228
    println!("Byte at index 1: {}", bytes[1]); // Output: 189
    println!("Byte at index 2: {}", bytes[2]); // Output: 160
    // These bytes together form the first character '你'

    // method 4
    println!("method 4 --> alternative 3 method ");
    let first_char_bytes = &greeting.as_bytes()[0..3];
    let first_char = String::from_utf8_lossy(first_char_bytes);
    println!("{}", first_char);
}
