use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);

    let data = buffered.lines().next().unwrap().unwrap();
    let mut re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = re.captures_iter(&data).map(|cap| {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        println!("{} * {} = {}", a, b, a * b);
        a * b
    }).sum();
    println!("Sum: {}", sum);

    re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let (cond_sum,_): (i32, bool) = re.captures_iter(&data).fold((0,true),|(sum, flag),cap| {
        let mut f = flag;
        let mut s = sum;
        if cap[0].eq("do()") {
            println!("do()");
            f = true;
        } else if cap[0].eq("don't()") {
            println!("don't()");
            f = false;
        } else if flag {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            println!("{} * {} = {}", a, b, a * b);
            s += a * b
        }
        (s, f)
    });
    println!("Conditional Sum: {}", cond_sum);
}
