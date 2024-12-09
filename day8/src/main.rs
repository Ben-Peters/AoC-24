use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<String> = Vec::new();
    let mut frequencies: HashMap<char,Vec<(usize,usize)>> = HashMap::new();
    for line in buffered.lines() {
        data.push(line.unwrap());
        for (i, char) in data.last().unwrap().chars().enumerate() {
            if char == '.' {
                continue;
            }
            if frequencies.contains_key(&char) {
                frequencies.get_mut(&char).unwrap().push((data.len()-1, i));
            } else {
                frequencies.insert(char, vec![(data.len()-1, i)]);
            }
        }
    }
    for (&frequency, locs) in frequencies.iter() {
        println!("{}: {:?}", frequency, locs);
        for (i,loc) in locs.iter().enumerate() {
            for (j, loc2) in locs.iter().enumerate().skip(i+1) {
                if i != j {
                    println!("{:?}: {:?}", loc, loc2);
                    let dist = ((loc2.0 as isize - loc.0 as isize), (loc2.1 as isize - loc.1 as isize));
                    println!("dist: {:?}", dist);
                    let mut scale = 0;
                    while loc.0 as isize - (dist.0*scale) >= 0 && ((loc.0 as isize - (dist.0*scale)) as usize) < data.len() && 
                        loc.1 as isize - (dist.1*scale) >= 0 && ((loc.1 as isize - (dist.1*scale)) as usize) < data[0].len() {
                        data[loc.0 - (dist.0*scale) as usize].replace_range(
                            (loc.1 as isize - (dist.1*scale))as usize.. 
                                (loc.1 as isize - (dist.1*scale)) as usize+1, "#");
                        scale += 1;
                    }
                    scale = 0;
                    while loc2.0 as isize + (dist.0*scale) >= 0 && ((loc2.0 as isize + (dist.0*scale)) as usize) < data.len() && 
                        loc2.1 as isize + (dist.1*scale) >= 0 && ((loc2.1 as isize + (dist.1*scale)) as usize) < data[0].len() {
                        data[loc2.0 + (dist.0*scale) as usize].replace_range(
                            (loc2.1 as isize + (dist.1*scale))as usize.. 
                                (loc2.1 as isize + (dist.1*scale)) as usize+1, "#");
                        scale += 1;
                    }
                }
            }
        }
    }
    for line in data.iter() {
        println!("{}", line);
    }
    let sum = data.iter().map(|s| s.chars().filter(|c| *c == '#').count()).sum::<usize>();
    println!("sum: {}", sum);
}
