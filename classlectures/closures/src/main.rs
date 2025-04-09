fn essence_example_closure() {
    let x = 21;
    let get_answer = |y: i32| x + y;
    println!("{:?}", get_answer(21));
}
/*
fn using_function_as_variable() {
    // Regular function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Function pointer
    let f = add;

    // Closure with explicit types
    let f = |x: i32, y: i32| { x + y };

    // Simplified closure
    let f = |x: i32, y: i32| x + y;

    // Closure with inferred types
    let f = |x, y| x + y;
    
    let result = f(1, 2);
    println!("{}", result);  // Output: 3
} 

fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug { //impl Debug <T:Debug>
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }
} */

fn main() {
    essence_example_closure();
    essence_example_closure();
}
