fn main() {
    let mut x = 5;
    println!("The value of x is:{} ",x);
    x =7;
    println!("The value of x is:{} ",x);
    let name = "Mukesh Amaresh Thakur";
    println!("Name is : {}",name);
    let month = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    println!("Months : {} ",month[11]);
    for number in (1..4).rev(){
        println!("{}!",number);
    }
}
