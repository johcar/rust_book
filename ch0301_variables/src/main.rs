

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup :(f32, u8, u32) = (12.13, 0xff, 1500);

    println!("this is 0 {}", tup.0);
    println!("this is 1 {}", tup.1);
    println!("this is 2 {}", tup.2);
}
