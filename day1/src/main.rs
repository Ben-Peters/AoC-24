use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn main() {
    // Import the data from the ./Data/input.txt file
    let path = Path::new("./Data/input.txt");
    let input = File::open(&path);
    match input {
        Ok(input) => {
            let buffered = io::BufReader::new(input);

            let mut list1: Vec<u32> = Vec::new();
            let mut list2: Vec<u32> = Vec::new();
        
            for line in buffered.lines() {
                match line {
                    Ok(line) => {
                        let nums = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                
                        assert!(nums.len() == 2);
                        list1.push(nums[0]);
                        list2.push(nums[1]);
                    },
                    Err(e) => panic!("Error reading line: {}", e),
                }
            }
            list1.sort();
            list2.sort();
            let mut diff = 0;
            diff = list1.iter().zip(list2.iter())
                        .map(|(l1,l2)| l1.abs_diff(*l2))
                        .sum();
            println!("Diff: {}", diff);

            let mut simScore = 0;
            simScore = list1.iter()
            .map(|l1| *l1 * (list2.iter().filter(|x| **x == *l1).count()).try_into().unwrap_or(0))
            .sum();
            println!("SimScore: {}", simScore);            

        },
        Err(e) => panic!("Error opening file: {}", e),
    }
}
