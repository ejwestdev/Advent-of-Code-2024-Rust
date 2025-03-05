use std::{fs, str::Lines};
fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path)
                .expect("File Error");
    let reports = contents.lines();
    part_a(&reports, &contents);
    part_b(&reports, &contents);

}
fn part_b(reports: &Lines<'_>, contents: &String) {
    let mut checker: Vec<Vec<i32>> = Vec::new();
    for i in reports.clone() {
        let splits = i.split_whitespace();
        let mut report: Vec<i32> = Vec::new();
        for s in splits {
            report.push(s.parse::<i32>().expect("Line error"));
        }

        let ascending: bool = report[1] > report[0];

        for window in report.windows(2) {
            let lvl = window[1];
            let prev_lvl = window[0];
            let diff = lvl - prev_lvl;
            let diff_size_good = (diff >= 1 && diff <= 3) || (diff <= -1 && diff >= -3);
            if !diff_size_good
            {
                checker.push(report);
                break;
            }
            if ascending && lvl < prev_lvl {
                checker.push(report);
                break;
            }
            if !ascending && lvl > prev_lvl {
                checker.push(report);
                break;
            }
        }
    }

    let sum = contents.lines().count() - checker.len();
    println!("Answer to Part B: {}", sum);
}


fn part_a(reports: &Lines<'_>, contents: &String) {
    let mut checker: Vec<Vec<i32>> = Vec::new();
    for i in reports.clone()
    {
        let splits = i.split_whitespace();
        let mut report: Vec<i32> = Vec::new();
        for s in splits {
            report.push(s.parse::<i32>().expect("Line error"));
        }

        let ascending: bool = report[1] > report[0];

        for window in report.windows(2) {
            let lvl = window[1];
            let prev_lvl = window[0];
            let diff = lvl - prev_lvl;
            let diff_size_good = (diff >= 1 && diff <= 3) || (diff <= -1 && diff >= -3);
            if !diff_size_good
            {
                checker.push(report);
                break;
            }
            if ascending && lvl < prev_lvl {
                checker.push(report);
                break;
            }
            if !ascending && lvl > prev_lvl {
                checker.push(report);
                break;
            }
        }
    }

    let sum = contents.lines().count() - checker.len();
    println!("Answer to Part A: {}", sum);
}
