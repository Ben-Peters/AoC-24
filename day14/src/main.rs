use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<((i32,i32),(i32,i32))> = Vec::new();
    let dimentions = (101,103);
    for line in buffered.lines() {
        // Format is p=0,4 v=3,-3
        // Get the position and velocity and store them as tuples
        let l= line.unwrap();
        //println!("{}", l);
        //println!("{}", l.split_whitespace().nth(0).unwrap().replace("p=",""));
        let p: Vec<i32> = l.split_whitespace().nth(0).unwrap().replace("p=","").split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let v: Vec<i32> = l.split_whitespace().nth(1).unwrap().replace("v=","").split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        data.push(((p[0],p[1]),(v[0],v[1])));
    }
    let mut final_pos: Vec<(i32,i32)> = Vec::new();
    for i in 0..10000000{
        final_pos = data.iter().map(|(p,v)| {
                //println!("({}+{})%{}={}", p.1, v.1 * 5, dimentions.1, (((p.1+(v.1 * 5))%dimentions.1)+dimentions.1)%dimentions.1);
                // Have to use ((a%b)+b)%b to get the correct value
                ((((p.0+(v.0 * i))%dimentions.0)+dimentions.0)%dimentions.0, (((p.1+(v.1 * i))%dimentions.1)+dimentions.1)%dimentions.1)
            }).collect::<Vec<(i32,i32)>>();
        let pos_hash = final_pos.iter().cloned().collect::<HashSet<(i32,i32)>>();
        if pos_hash.len() >= 500{
            println!("Time: {}", i);
            for y in 0..dimentions.1{
                for x in 0..dimentions.0{
                    if pos_hash.contains(&(x,y)){
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("--------------------------------");
        }
    }
    let mut quad_a = 0;
    let mut quad_b = 0;
    let mut quad_c = 0;
    let mut quad_d = 0;

    for pos in final_pos{
        if pos.0 < dimentions.0/2 && pos.1 < dimentions.1/2 {
            quad_a += 1;
        } else if pos.0 > dimentions.0/2 && pos.1 < dimentions.1/2 {
            quad_b += 1;
        } else if pos.0 < dimentions.0/2 && pos.1 > dimentions.1/2 {
            quad_c += 1;
        } else if pos.0 > dimentions.0/2 && pos.1 > dimentions.1/2 {
            quad_d += 1;
        }
    }

    println!("Security Score: {}", quad_a*quad_b*quad_c*quad_d);
    
}
