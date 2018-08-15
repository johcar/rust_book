use std::fs::File;

fn main() {
    // unwrap returns value if Result is OK, if it is Err it will call the panic! macro
    let f = File::open("hello.txt").unwrap();

    // expect is like unwrap but will let you specify the error/panic message
    let f = File::open("hello_exp.txt").expect("Failed to open hello_exp.txt");
}