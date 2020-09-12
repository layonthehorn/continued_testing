use std::cmp::min;

pub fn reverse_dismantle_vec(mut vector: Vec<i32>) {
    while vector.len() > 0 {
        match vector.pop() {
            Some(value) => println!("Found value: {}", value),
            None => {}
        };
    }
}

pub fn forward_dismantle_vec(mut vector: Vec<i32>) {
    while vector.len() > 0 {
        let value = vector.remove(0);
        println!("Found value: {}", value);
    }
}

// selection sort in Rust
pub fn selection_sort(mut vector: Vec<i32>) {
    for index in 0..vector.len() {
        let mut min_value = index;

        for next in index + 1..vector.len() {
            if vector[min_value] > vector[next] {
                min_value = next;
            }
        }
        // selection switches after checking all values
        vector.swap(min_value, index);
    }
    print_vec(vector);
}

// bubble sort in Rust
pub fn bubble_sort(mut vector: Vec<i32>) {
    for index in 0..vector.len() {
        let mut min_value = index;

        for next in index + 1..vector.len() {
            // swaps them as soon as it finds a larger value
            if vector[min_value] > vector[next] {
                vector.swap(min_value, next);
            }
        }
    }
    print_vec(vector);
}

pub fn quick_sort(vector: &mut Vec<i32>,low: i32, high: i32) {
    if low < high{
        let pi = partition(vector, low, high);
        quick_sort(vector, low, pi -1);
        quick_sort(vector, pi + 1, high);

    }
}

fn partition(mut vector: &mut Vec<i32>, low: i32, high: i32) -> i32{
    let pivot = vector[high as usize];
    let mut number = (low - 1);
    for index in low..high{
        if vector[index as usize] < pivot{
            number += 1;
            vector.swap(number as usize, index as usize);
        }
    }
    vector.swap((number + 1) as usize, high as usize );
    number + 1
}

pub fn print_vec(vector: Vec<i32>) {
    // just prints the values that are now sorted
    for value in vector.iter() {
        print!("{} ", value)
    }
    println!()
}
