fn main() {
    let s1 = String::from("Daddu");
    let lambayai = kitana_lamba_re(&s1);
    println!("s1 -> \t{}", s1);
    println!("lambayai -> \t{lambayai}");
    let mut s2 = s1.clone();
    badla(&mut s2);
    println!("{}", s2);
    let s3 = & s2;
    println!("s3: {}", s3);
}

fn kitana_lamba_re(s: &String) -> usize {
    s.len()
}

fn badla(ranatunga: &mut String) {
    ranatunga.push_str(" , to kaise ho aap");
}
