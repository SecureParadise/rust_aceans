// assert_eq!(l_v, r_v)
// assert!(condition)
fn main() {
    // let num:i32 = 1__0_2_4;
    let num = 0xFF;
    println!("Number is {} ",num);
    println!("Number type is  {} ",type_of(&num));
    print_a_z();

    range_rover();
}
// function to get Type of the Data
fn type_of<T>(_:&T) -> String{
    format!("{}",std::any::type_name::<T>())
}
fn print_a_z(){
    for character in 'a'..='z' {
        println!("{}",character as i32);
    }
}
fn range_rover(){
    use std::ops::{Range,RangeInclusive};
    assert_eq!((1..5),Range{start:1,end:5});
    assert_eq!((1..=5),RangeInclusive::new(1,5));

}
