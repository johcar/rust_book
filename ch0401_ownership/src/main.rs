fn scope_of_s_literal() {     // s is not valid here, it is not yet declared
    let s = "hello";// s is valid from this point 
    println!("String literal is {}", s);                // do stuff with s 
}                   // this scope is now over and s is no longer valid

fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literal to a String

    println!("String on heap is now {}", s);
}

fn scope_of_s_string_type() {
    let s = String::from("hello"); // s is valid from this point forward
    println!("String of type String is {}", s);
    // do stuff with s
}   // this scope is now over, and s is no
    // longer valid and heap memory will be returned by the drop function in the String type

fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn main() {
    scope_of_s_literal();
    string_type();
    scope_of_s_string_type();
    ownership_and_functions();
}