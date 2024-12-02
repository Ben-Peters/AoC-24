use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn check_valid (report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.windows(2)
    .map(|w| 
        w[1] - w[0])
        .collect::<Vec<i32>>();
    (diffs.iter().all(|d| *d > 0) ||
    diffs.iter().all(|d| *d < 0)) &&
    diffs.iter().all(|d| d.abs() > 0 && d.abs() <= 3)
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);

    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in buffered.lines() {
        let nums = line.unwrap().split_whitespace()
        .map(|s| 
            s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
        data.push(nums);
    }

    let mut valid: Vec<&Vec<i32>> = Vec::new();
    let mut invalid: Vec<&Vec<i32>> = Vec::new();
    for report in &data {
        if check_valid(report){
            valid.push(report);
        }else{
            invalid.push(report);
        }
    }

    println!("NumSafe (part 1): {}", valid.len());
    // I'm sure this can be done more efficiently but I struggled to figure out the best way to do it
    for report in invalid.iter() {
        if report.iter().enumerate().map(|(i,_)| {
            let mut report_copy = report.clone().to_vec();
            report_copy.remove(i);
            check_valid(&report_copy)
        }).any(|b| b){
            valid.push(report);
        }
    }
    println!("NumSafe (part 2): {}", valid.len());

}
