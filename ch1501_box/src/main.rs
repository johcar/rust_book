use List::{Cons, Nil};


// listing 15-3 (won't compile because: 
//  |enum List {
//  | ^^^^^^^^^ recursive type has infinite size
//  |     Cons(i32, List),
//  |               ---- recursive without indirection)

//enum List {
//    Cons(i32, List),
//    Nil,
//}
//fn main() {
//    let list = Cons(1, Cons(2, Cons(3, Nil)));
//}


// listing 15-5, will compile because we use a box

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
            
}