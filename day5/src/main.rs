use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn check_valid (update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> (bool, (usize,usize)) {
    for (i, page) in update.iter().enumerate() {
        if rules.contains_key(page) {
            for other_page in update[..i].iter() {
                if rules[page].contains(other_page) {
                    println!("{} cannot be after {}", page, other_page);
                    println!("Invalid update: {:?}", update);
                    return (false, (i, update.iter().position(|&x| x == *other_page).unwrap()));
                }
            }
        }
    }
    return (true, (0,0));
}

fn main() {
    let input_path = Path::new("./data/input.txt");
    let rule_path = Path::new("./data/rules.txt");
    let input = File::open(&input_path).unwrap();
    let rules_f = File::open(&rule_path).unwrap();
    let mut buffered = io::BufReader::new(input);

    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in buffered.lines() {
        data.push(
            line.unwrap().split(",").map(|s| {
                s.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        );
    }

    buffered = io::BufReader::new(rules_f);
    let mut rules_input: Vec<(i32, i32)> = Vec::new();
    for line in buffered.lines() {
        let rule = line.unwrap().split("|").map(|s| {
            s.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>();
        rules_input.push((rule[0], rule[1]));
    }

    // Rules will take the form of num followed by a list of pages that cannot be after it
    let mut rules: HashMap<i32,Vec<i32>> = HashMap::new();
    for (num, page) in rules_input {
        rules.entry(num).or_insert(Vec::new()).push(page);
    }

    let mut sum = 0;
    let mut invalid: Vec<(Vec<i32>,(usize,usize))> = Vec::new();
    for update in data.iter() {
        // Check that for each update no page is printed after a page that it cannot be printed after
        let (valid,pages) = check_valid(update, &rules);
        if valid {
            println!("Valid update: {:?}", update);
            //add the middle value of the update
            sum += update[(update.len()-1)/2];
        } else {
            invalid.push((update.clone(), pages));
        }
    }
    println!("Sum of valid updates: {}", sum);

    sum = 0;
    // Part 2

    // This is just about the worst way to do this but it works and does't take too long
    // It could be improved significantly by inserting into the correct position and sliding
    // instead of swapping
    for (update, pages) in invalid.iter() {
        let mut valid = false;
        let mut new_update = update.clone();
        let mut new_pages = pages.clone();
        while (!valid){
            // Swap the violating pages, check and then keep doing that
            new_update.swap(new_pages.0, new_pages.1);
            (valid, new_pages)  = check_valid(&new_update, &rules);
            
        }
        sum += new_update[(new_update.len()-1)/2];
    }
    println!("Sum of valid updates after correction: {}", sum);

}
