fn broken_example() {
    // Generic in functions
    // Often used case scenario, example not working
    // but just to give you general idea how it may look
    // with next week knowledge we will make it work
    // For curious students: there are two problems
    // 1) When you are using generic data type, question is how long that data will live
    // 2) What kind of operation this data type is supposed to support

    fn largest<T>(list: &[T]) -> T { 
         let mut largest = list[0];
         for &item in list.iter() {
             if item > largest {
                 largest = item;
             }
         }
         largest
    }
}

fn main() {
    broken_example()
}
