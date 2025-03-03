//use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("File error");

    let mut words_vec: Vec<i32> = Vec::new();
    let words = contents.split_whitespace();
    for word in words {
        words_vec.push(word.trim().parse().expect("Number please!"));
    }

    let mut right_ids: Vec<&i32> = Vec::new();
    let mut left_ids: Vec<&i32> = Vec::new();
    let mut sum = 0;

    for (i, value) in words_vec.iter().enumerate() {
        if (i+1)%2 == 0
        {
            right_ids.push(value);
        }
        else {
            left_ids.push(value);
        }
    }

    println!("{:?}", left_ids);
    let mut left_counts: HashMap<i32,i32> = HashMap::new(); 

    for &value in &left_ids {
        *left_counts.entry(*value).or_insert(0) += 1;
    }
    println!("{:?}", left_counts);
    for &value in &right_ids {
        if let Some(count) = left_counts.get(&value) {
            println!("{} {}", value, count);
            sum += value * count;
        }
    }

    println!("The answer is: {sum}");

}
