// Write a function first_word that takes a string and returns the first word as a slice.

fn main() {
    // let s = String::from("hello rust");
    // let word = first_word(&s);
    // println!("First word: {}", word);

    let mut sentence = String::from("koi mil gaya mera dila gya koi mil gaya mil hi gya");
    // let mut sentence = "koi mil gaya mera dila gya koi mil gaya mil hi gya";
    // let sentence = "OK";
    // let sentence = "";
    // let sentence = " ";
    // println!("len of sentence is {}", sentence.len());
    // let no_of_word_count = split_whitespace(&sentence);
    println!(
        "No of word count of->{}->is {}",
        sentence,
        split_whitespace(&sentence)
    );
    println!(
        "First word of->{}->is \t  {}\t",
        sentence,
        first_word_slice(&sentence)
    );
    // println!("Modified sentence is {}", first_word_replace(&mut sentence,"Lo_ji_ho_gya_replacement"));
    first_word_replace(&mut sentence, "Lo_ji_ho_gya_replacement");
    println!("Modified sentence is: {}", sentence);
}

fn split_whitespace(sentence: &str) -> usize {
    let sentence_byte = sentence.as_bytes();
    let mut count = 0;
    if sentence.len() > 1 {
        count = 1;
    }
    for (_, &item) in sentence_byte.iter().enumerate() {
        if item == b' ' {
            if sentence.len() == 1 {
                return count;
            }
            count += 1;
        }
    }
    return count;
}

fn first_word_slice(sentence: &str) -> &str {
    let sentence_byte = sentence.as_bytes();
    for (i, &item) in sentence_byte.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }
    return sentence;
}

// fn first_word_replace(s: &mut String, replacemet: &str) -> &str {
//     // write logic here
//     if let Some(pos) = s.find(' ') {
//         let rest = &s[pos..];
//         *s = format!("{}{}", replacemet, rest);
//     } else {
//         *s = String::from(replacemet);
//     }
// }
fn first_word_replace(s: &mut String, replacement: &str) {
    if let Some(pos) = s.find(' ') {
        let rest = &s[pos..];
        *s = format!("{}{}", replacement, rest);
    } else {
        *s = String::from(replacement);
    }
}
