fn main() {
    let series_upto: u32 = 4;
    fibonacci_number_series(series_upto);
}

fn fibonacci_number_series(n: u32) {
    if n == 0 {
        print!("No term to show");
    } else if n == 1 {
        print!("0");
    } else {
        let mut a = 0;
        let mut b = 1;
        print!("{a} ,{b} ,");
        for _ in 2..n {
            let sum = a + b;
            print!("{} , ", sum);
            a = b;
            b = sum;
        }
    }
}
