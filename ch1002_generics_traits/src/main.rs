// 10-15 from book
//fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//   let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//   largest
//}

//fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];
//
//   let result = largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//
//    let result = largest(&char_list);
//    println!("The largest char is {}", result);
//}


// this version return a reference to the largest value instead of a copy as in listing 10-15 above
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest_index = 0;
    let mut largest = list[0];
    for (index, &item) in list.iter().enumerate() {
        if item > largest {
            largest_index = index;
            largest = item;
        }
    }

    &list[largest_index]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
