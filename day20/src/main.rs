use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn find_path(start: (usize,usize), end: (usize,usize), dimentions: (usize,usize), walls: HashSet<(usize,usize)>) -> Vec<Vec<usize>>{
    let mut min_dist: Vec<Vec<usize>> = vec![vec![usize::MAX; dimentions.1]; dimentions.0];

    let mut queue: Vec<((usize,usize),usize)> = Vec::new();
    min_dist[start.0][start.1] = 0;
    queue.push((start,0));
    while let Some(((i,j), dist)) = queue.pop(){
        if i == end.0 && j == end.1{
            println!("Distance: {}", dist);
            for i_p in 0..dimentions.0{
                for j_p in 0..dimentions.1{
                    if i_p == i && j_p == j{
                        print!("X");
                        continue;
                    }
                    if min_dist[i_p][j_p] != usize::MAX{
                        print!("O");
                        continue;
                    }
                    print!("{}", if walls.contains(&(i_p,j_p)){'#'} else{'.'});
                }
                println!("");
            }
            println!("-------------------------");
            return min_dist;
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
            if walls.contains(&(new_i,new_j)){
                // Wall
                continue;
            }
            if new_dist < min_dist[new_i][new_j]{
                min_dist[new_i][new_j] = new_dist;
                queue.push(((new_i,new_j),new_dist));
            }
        }
        queue.sort_by(|(_,a),(_,b)| b.cmp(&a));
    }
    return min_dist;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut walls: HashSet<(usize,usize)> = HashSet::new();
    let mut start = (0,0);
    let mut end = (0,0);
    let mut dimentions = (0,0);
    let shortcut_thresh = 100;
    for (i,line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        dimentions.0 += 1;
        dimentions.1 = l.len();
        for (j,c) in l.chars().enumerate(){
            if c == '.'{
                // do nothing this is an empty space
            } else if c == 'S'{
                start = (i,j);
            } else if c == 'E'{
                end = (i,j);
            } else if c == '#'{
                walls.insert((i,j));
            }
        }
    }
    let min_dist = find_path(start, end, dimentions, walls);
    let mut part_1 = 0;
    let mut part_2 = 0;
    for (i,row) in min_dist.iter().enumerate(){
        for (j,&dist) in row.iter().enumerate(){
            if dist != usize::MAX{
                // This is a spot on the path. Try to see if their is a shortcut
                for &dir in [(0,2),(2,0),(-2,0),(0,-2)].iter(){
                    // Part 1
                    if (i as isize + dir.0) < 0 || (j as isize + dir.1) < 0 || (i as isize + dir.0) >= dimentions.0 as isize || (j as isize + dir.1) >= dimentions.1 as isize{
                        // Out of bounds
                        continue;
                    }
                    let new_i = (i as isize + dir.0) as usize;
                    let new_j = (j as isize + dir.1) as usize;
                    if min_dist[new_i][new_j] != usize::MAX && min_dist[new_i][new_j] > dist+2{
                        // This is a shortcut
                        let savings = min_dist[new_i][new_j]-(dist+2);
                        //println!("Shortcut found at ({},{}) to ({},{}) saving {}", i,j,new_i,new_j, savings);
                        if savings >= shortcut_thresh{
                            part_1 += 1;
                        }
                    }
                }
                // Part 2
                // find all spots less than 20 steps away and see which is the highest.
                // if that is greater than the thersh then record it
                for i_offset in -20..21 as i32 {
                    for j_offset in -(20 - (i_offset as i32).abs())..(20 - (i_offset as i32).abs()+1){
                        if i as i32 + i_offset < 0 || j as i32 + j_offset < 0 || (i as i32 + i_offset) as usize >= dimentions.0 || (j as i32 + j_offset) as usize >= dimentions.1{
                            // Out of bounds
                            continue;
                        }
                        let new_i = (i as i32 + i_offset) as usize;
                        let new_j = (j as i32 + j_offset) as usize;

                        if min_dist[new_i][new_j] != usize::MAX && min_dist[new_i][new_j] > dist + (i_offset.abs() + j_offset.abs()) as usize{
                            // This is a shortcut
                            let savings = (min_dist[new_i][new_j] - (dist + (i_offset.abs() + j_offset.abs()) as usize));
                            //println!("Shortcut found at ({},{}) to ({},{}) saving {}", i,j,new_i,new_j, savings);
                            if savings >= shortcut_thresh{
                                part_2 += 1;
                            }
                        }
                    }

                }
            }
        }
    }
    println!("Part_1 found: {}", part_1);
    println!("Part_2 found: {}", part_2);
}
