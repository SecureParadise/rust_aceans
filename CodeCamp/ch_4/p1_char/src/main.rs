fn main() {
    // let var = 32;
    func_1();
    boolyi();
}

fn func_1() {
    use std::mem::size_of_val;
    let c1 = 'a';
    println!("Size of variable is {}", size_of_val(&c1),);
    // assert_eq!(size_of_val(&c1),4);
}

fn boolyi() {
    println!("boolyi is called");
    let t = true;
    if !t {
        println!("Dekho dekho");
    }
}
fn unit_type() {
    use std::mem::size_of_val;
    let unit: () = ();
    println!("size of unit is {}",size_of_val(&unit));
    // assert!(size_of_val(&unit) == 0);

    println!("Success!");
    
}

// Make it work with two ways
fn expression() {
    let v:i32 = {
        let mut x:i32 = 1;
        x += 2;
        x //x is expression -> returned automatically
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }