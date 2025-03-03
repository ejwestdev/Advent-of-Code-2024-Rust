//use std::env;
use std::fs;

fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("File error");

    let mut words_vec: Vec<i32> = Vec::new();
    let words = contents.split_whitespace();
    for word in words {
        words_vec.push(word.trim().parse().expect("Number please!"));
    }

    let mut left_col: Vec<&i32> = Vec::new();
    let mut right_col: Vec<&i32> = Vec::new();

    for (i, value) in words_vec.iter().enumerate() {
        if (i+1)%2 == 0
        {
            left_col.push(value);
        }
        else {
            right_col.push(value);
        }
    }
    left_col.sort();
    right_col.sort();
    let mut sum: i32 = 0;

    for i in 0..left_col.len() {
        let difference = (left_col[i] - right_col[i]).abs();
        sum += difference;
    }
    println!("{sum}");
}
