use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn try_num (target: i64, nums: &[i64], use_concat: bool) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }
    if target < 0 {
        return false;
    }
    let num = nums[nums.len()-1];
    let dif = target-num;
    let div = target/num;
    let mut b_div = false;
    let mut b_sub = false;
    let mut b_con = false;
    if target % num == 0 {
         b_div = try_num(div, &nums[..nums.len()-1], use_concat);
    }
    b_sub = try_num(dif, &nums[..nums.len()-1], use_concat);

    let str_num = num.to_string();
    let str_tgt = target.to_string();
    if use_concat && str_tgt.len() >= str_num.len() && str_tgt[str_tgt.len()-str_num.len()..] == str_num{
        // println!("{} || {}", str_tgt, str_num);
        let les_concat = if str_tgt[..str_tgt.len()-str_num.len()].len() > 0 {
            str_tgt[..str_tgt.len()-str_num.len()].parse::<i64>().unwrap()
        }else{
            0
        };
        if les_concat == 0 {
            b_con = true;
        }else {
            b_con = try_num(les_concat, &nums[..nums.len()-1], use_concat);
        }
    }

    return b_div || b_sub || b_con;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<(i64, Vec<i64>)> = Vec::new();
    for line in buffered.lines() {
        let l = line.unwrap();
        let target = l.split(":").nth(0).unwrap().parse::<i64>().unwrap();
        println!("target: {}", target);
        let nums = l.split(":").nth(1).unwrap().split_whitespace().map(|s| {
            s.parse::<i64>().unwrap()}).collect::<Vec<i64>>();
        data.push((target, nums));
    }
    println!("{:?}", data);
    let mut sum = 0;
    for (target, nums) in data.iter() {
        println!("{}: {:?}", target, nums);
        if try_num(*target, nums, false){
            sum += target;
        }
    }
    println!("sum: {}", sum);

    sum = 0;
    for (target, nums) in data.iter() {
        // println!("{}: {:?}", target, nums);
        if try_num(*target, nums, true){
            sum += target;
        }
    }
    println!("sum: {}", sum);
}
