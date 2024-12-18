use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn find_path(start: (usize,usize), bytes: Vec<(usize,usize)>) -> (usize, Vec<(usize,usize)>){
    let dimentions = (71,71);
    let end = (dimentions.0-1, dimentions.1-1);
    let mut min_dist: Vec<Vec<usize>> = vec![vec![usize::MAX; dimentions.1]; dimentions.0];

    let mut queue: Vec<((usize,usize),usize, Vec<(usize,usize)>)> = Vec::new();
    min_dist[0][0] = 0;
    let path = Vec::new();
    queue.push(((0,0),0,path.clone()));
    while let Some(((i,j), dist,mut path)) = queue.pop(){
        path.push((i,j));
        if i == end.0 && j == end.1{
            println!("Distance: {}", dist);
            for i_p in 0..dimentions.0{
                for j_p in 0..dimentions.1{
                    if i_p == i && j_p == j{
                        print!("X");
                        continue;
                    }
                    if path.contains(&(i_p,j_p)){
                        print!("O");
                        continue;
                    }
                    print!("{}", if bytes.contains(&(i_p,j_p)){'#'} else{'.'});
                }
                println!("");
            }
            println!("-------------------------");
            return (dist, path);
        }
        for dir in [(0,1),(1,0),(-1,0),(0,-1)].iter(){
            let new_i = i as isize + dir.0;
            let new_j = j as isize + dir.1;
            if new_i < 0 || new_j < 0 || new_i >= dimentions.0 as isize || new_j >= dimentions.1 as isize{
                // Out of bounds
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let new_dist = dist + 1;
            if bytes.contains(&(new_i,new_j)){
                // corrupted space
                continue;
            }
            if new_dist < min_dist[new_i][new_j]{
                min_dist[new_i][new_j] = new_dist;
                queue.push(((new_i,new_j),new_dist, path.clone()));
            }
        }
        queue.sort_by(|(_,a,_),(_,b,_)| b.cmp(&a));
        //println!("{:?}", queue);
    }
    return (0,path);
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut byte_pos: Vec<(usize,usize)> = Vec::new();
    for line in buffered.lines() {
        let l = line.unwrap();
        byte_pos.push((l.split(",").nth(1).unwrap().parse::<usize>().unwrap(), 
                        l.split(",").nth(0).unwrap().parse::<usize>().unwrap()));
    }

    let (part1,mut path) = find_path((0,0), byte_pos[0..1024].to_vec());

    println!("Part 1: {}", part1);

    for i in 1024..byte_pos.len(){
        let mut dist = 0;
        if path.contains(&byte_pos[i-1]){
            println!("Potential blockage at position: {},{}", byte_pos[i-1].1, byte_pos[i-1].0);
            (dist,path) = find_path((0,0), byte_pos[0..i].to_vec());
            if dist == 0{
                println!("Blocked by item at position: {},{}", byte_pos[i-1].1, byte_pos[i-1].0);
                break;
            }
        }
    }
}
