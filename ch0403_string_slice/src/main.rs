fn main() {
    let string = String::from("hello world");
    let string_after = first_word(&string[0..3]);
    println!("{}", string_after);
}


fn first_word(s: &str) -> &str {
    println!("{}", s);
    
    s
}