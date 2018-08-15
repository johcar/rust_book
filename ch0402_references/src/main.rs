fn borrower(string: &mut String) {
    println!("borrowed string is {}",  string);
    string.push_str(", world");
    println!("borrowed string is now changed to {}",  string);
}

fn dangling_pointer() {
    let reference_to_nothing = dangle();
}


fn dangle() -> String {
    let s = String::from("hello");

    //&s // wont work because it will be a dangling pointer
    s // works because ownership is moved out
}

fn main() {
    // borrowing mutable reference
    println!("Hello, world!");
    let mut s = String::from("hello hey");
    borrower(&mut s);
    

    //let m = &mut s; cant have 2 mutable refs in one scope
    let r = &mut s;
    println!("{}", r);
    

    dangling_pointer();
}
