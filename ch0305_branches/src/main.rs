fn main() {
    
    // EX 1
    //let number = 3; 
    let number = 7;
    
    
    if number < 5 {
        println!("connection is true");
    } else {
        println!("connection is false");
    }

    // EX 2
    // shadow by using let again
    let number = 3;

    // cant use if number{} because rust will not try to convert nonbooleans to boolean
    // boolean is a must here
    if number != 0 {
        println!("number was something other than zero");
    }

    // EX 3
    // again shadowing
    let number = 6; 

    // else if example
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // EX 4
    // you can assign a value to a variable with an if expression
    // however potential results must be of the same type
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // EX 5 
    // loop
    //loop {
     //   println!("again!");
    //}
    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
    // as a for loop using reveresed range instead
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    // looping through a collection
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
