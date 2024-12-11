use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn count_blink(stone: i64, rem_blinks: i32, memory: &mut HashMap<(i64,i32),i64>) -> i64{
    let mut count: i64 = 0;
    if  let Some(chached_res) = memory.get(&(stone, rem_blinks)) {
        return *chached_res;
    }
    //println!("{:?} {}", stone, rem_blinks);
    if rem_blinks == 0 {
        //print!("{:?} ", stone);
        return 1;
    }
    let stone_str = stone.to_string();
    if stone == 0 {
        count = count_blink(1, rem_blinks-1, memory);
    } else if stone_str.len()%2 == 0{
        count = count_blink(stone_str[0..stone_str.len()/2].parse::<i64>().unwrap(), rem_blinks-1, memory) +
            count_blink(stone_str[stone_str.len()/2..].parse::<i64>().unwrap(), rem_blinks-1, memory);
    } else {
        count = count_blink(stone*2024, rem_blinks-1, memory);
    }
    memory.insert((stone, rem_blinks), count);
    return count;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<i64> = Vec::new();
    for line in buffered.lines() {
        data = line.unwrap().split_whitespace()
            .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    }
    println!("{:?}", data);
    let mut memory: HashMap<(i64,i32), i64> = HashMap::new();
    let mut count = 0;
    for stone in data.iter() {
        count += count_blink(*stone, 25, &mut memory);
    }
    println!("25: {:?}", count);
    count = 0;
    for stone in data.iter() {
        count += count_blink(*stone, 75, &mut memory);
    }
    println!("75: {:?}", count);

}
