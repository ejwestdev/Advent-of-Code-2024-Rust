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
    let mut valid_count = 0;
    
    for i in reports.clone() {
        let splits = i.split_whitespace();
        let mut report: Vec<i32> = Vec::new();
        for s in splits {
            report.push(s.parse::<i32>().expect("Line error"));
        }
        
        let mut can_be_valid = false;
        
        // For each index, try removing that element and check if the resulting report is valid
        for remove_idx in 0..report.len() {
            // Create a version of the report with the level at remove_idx removed
            let mut modified_report = report.clone();
            modified_report.remove(remove_idx);
            
            // Skip if too short
            if modified_report.len() < 2 {
                can_be_valid = true;
                break;
            }
            
            let ascending: bool = modified_report[1] > modified_report[0];
            let mut is_valid = true;
            
            for window in modified_report.windows(2) {
                let lvl = window[1];
                let prev_lvl = window[0];
                let diff = lvl - prev_lvl;
                let diff_size_good = (diff >= 1 && diff <= 3) || (diff <= -1 && diff >= -3);
                
                if !diff_size_good || 
                   (ascending && lvl < prev_lvl) || 
                   (!ascending && lvl > prev_lvl) {
                    is_valid = false;
                    break;
                }
            }
            
            if is_valid {
                can_be_valid = true;
                break;
            }
        }
        
        if can_be_valid {
            valid_count += 1;
        }
    }
    
    println!("Answer to Part B: {}", valid_count);
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
