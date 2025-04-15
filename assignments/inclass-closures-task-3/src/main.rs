fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers_map = vec![1, 2, 3];

    let doubled_map = process_vector_with_map(numbers_map.clone(), |x| {
        x * 2
    });

    let replaced_map = process_vector_with_map(numbers_map, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {
            0
        } else {
            x
        }
    });
    
    let numbers_for = vec![1, 2, 3];

    let doubled_for = process_vector_with_for_loop(numbers_for.clone(), |x| {
        x * 2
    });

    let replaced_for = process_vector_with_for_loop(numbers_for, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {
            0
        } else {
            x
        }
    });

    println!("Doubled using map and collect: {:?}", doubled_map);
    println!("Replaced using map and collect: {:?}", replaced_map);
    println!("Doubled using for loop: {:?}", doubled_for);
    println!("Replaced using forloop: {:?}", replaced_for);
}