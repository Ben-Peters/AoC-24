use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn count_trails (data: &Vec<Vec<i32>>, pos: (usize,usize)) -> Vec<(usize,usize)> {
    let mut reachable: Vec<(usize,usize)> = Vec::new();
    let cur_height = data[pos.0][pos.1];
    if cur_height == 9 {
        reachable.push(pos);
        return reachable;
    } else {
        if pos.0+1 < data.len() && data[pos.0+1][pos.1] == cur_height+1{
            reachable.extend(count_trails(data, (pos.0+1, pos.1)));
        }
        if (pos.0 as isize)-1 >= 0 && data[pos.0-1][pos.1] == cur_height+1{
            reachable.extend(count_trails(data, (pos.0-1, pos.1)));
        }
        if pos.1+1 < data[0].len() && data[pos.0][pos.1+1] == cur_height+1{
            reachable.extend(count_trails(data, (pos.0, pos.1+1)));
        }
        if (pos.1 as isize)-1 >= 0 && data[pos.0][pos.1-1] == cur_height+1{
            reachable.extend(count_trails(data, (pos.0, pos.1-1)));
        }
    }
    return reachable;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in buffered.lines() {
        data.push(line.unwrap().chars().
            map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()
        );
    }
    println!("{:?}", data);

    let pos = data.iter().enumerate().flat_map(|(i,row)| 
        {row.iter().enumerate().filter_map(move |(j, value)| 
            {if *value == 0 {
                Some((i,j))}
            else {None}
            })
        }).collect::<Vec<(usize,usize)>>();
    println!("{:?}", pos);
    //let mut reachable_locs: HashSet<(usize, usize)> = HashSet::new();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for p in pos.iter() {
        let reachable = count_trails(&data, *p);
        //reachable_locs = reachable_locs.union(&r).cloned().collect();
        println!("{:?}: {}", p, reachable.len());
         sum1 += reachable.iter().cloned().collect::<HashSet<(usize,usize)>>().len();
         sum2 += reachable.len();
    }


    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);

}
