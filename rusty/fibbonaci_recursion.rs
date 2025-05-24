fn main() {
    let series_upto: u32 = 6;
    for i in 0..series_upto {
        print!(" {} ",fibbonacci(i as u8));
    }
    println!();
}

fn fibbonacci(i: u8) -> u32 {
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else {
        fibbonacci(i-1) + fibbonacci(i-2)
    }
}
