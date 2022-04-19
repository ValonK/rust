use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let mut numbers = create_vector();
    let median = median_example(&mut numbers);
    println!("{}", median);
}

fn create_vector() -> Vec<i32>{
    let mut vector : Vec<i32> = Vec::new();
    vector.push(3);
    vector.push(1);
    vector.push(1);
    vector.push(1);
    vector.push(7);
    vector.push(2);
    vector.push(10);
    vector.push(4);
    vector.push(4);
    vector
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn median_example(int_vec: &mut Vec<i32>) -> i32 {
    int_vec.sort();
    let mid = int_vec.len() / 2;
    int_vec[mid]
}

fn mode_example(int_vec: &mut Vec<i32>) -> Option<i32> {
 
    let mut occurrences = HashMap::new();

    for value in int_vec {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    int_vec.iter().copied().max_by_key(|&n| {
        let count = int_vec.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}
