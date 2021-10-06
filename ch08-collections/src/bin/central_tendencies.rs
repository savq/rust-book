// Exercise 1:
// Given a list of integers, use a vector and return:
// * mean (the average value)
// * median (when sorted, the value in the middle position)
// * mode (the value that occurs most often; a hash map will be helpful here)
//
// TODO:
// - Read array from stdin
// use iter().sum() instead of for loop
// See if return statements are necessary in median fn

use std::collections::HashMap;

pub fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    for i in v {
        sum += *i as f64;
    }
    return sum / (v.len() as f64);
}

pub fn median(v: &Vec<i32>) -> f64 {
    let mut w = v.clone();
    w.sort();
    let middle = w.len() / 2; // integer division goes to floor
    if w.len() % 2 == 0 {
        return ((w[middle - 1] + w[middle]) as f64) / 2.0;
    } else {
        return w[middle] as f64;
    }
}

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for i in v {
        let c = hm.entry(i).or_insert(0);
        *c += 1;
    }

    let mut mode: i32 = 0; // Could be initialized to any value

    for (key, val) in &hm {
        if let Option::Some(mv) = hm.get(&mode) {
            if *val > *mv {
                mode = **key;
            }
        } else {
            mode = **key; // There's no mode yet
        }
    }
    mode
}

fn main() {
    let v: Vec<i32> = vec![362, 362, 242, 437, 362, 172, 376, 376, 246]; // 51
    println!("List: {:?}", v);
    println!("Mean: {}", mean(&v));
    println!("Median: {}", median(&v));
    println!("Mode: {}", mode(&v));
}
