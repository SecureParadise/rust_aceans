fn main() {
    // array,map,strings
    let sentence = String::from("Name RanaTungam");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
