use core::hash;
use std::fs::File;
use std::io::{self, BufRead};
use std::net;
use std::path::Path;
use std::collections::HashSet;


fn check_layout (data: Vec<String>, start_pos: (usize,usize)) -> (Vec<String>,Vec<(usize,usize)>,bool){
    let mut guard_dir: (isize, isize) = (-1,0);
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut trap = false;
    let mut horiz = data.clone();
    let mut guard_pos = start_pos.clone();
    path.push(guard_pos.clone());
    while guard_pos.0 < horiz.len() && guard_pos.1 < horiz[0].len() {
        let mut old_pos = guard_pos;
        if let Some(&op) = path.last() {
            if op != guard_pos {
                // Don't record when we stand in the same place but just trun
                path.push(guard_pos.clone());
            }
        }
        if guard_dir == (-1,0) {
            // let mut los = vert[guard_pos.1][..guard_pos.0].chars();
            let mut los = horiz.iter().map({
                |s| s.chars().nth(guard_pos.1).unwrap()
            }).collect::<String>()[..guard_pos.0].chars().collect::<String>();
            // println!("los: {:?}", los);
            let next = los.chars().rev().position(|c| c == '#');
            // println!("next: {:?}", next);
            if next.is_some(){
                guard_pos = (guard_pos.0 - (next.unwrap()), guard_pos.1);
                guard_dir = (0,1);
                // change each character we walk past to be an X
                for i in guard_pos.0..old_pos.0 {
                    horiz[i].replace_range(guard_pos.1..old_pos.1+1, "X");
                }
                // println!("guard_pos: {:?}", guard_pos);
            }else{
                // we would walk off the board
                for i in 0..guard_pos.0+1 {
                    horiz[i].replace_range(guard_pos.1..old_pos.1+1, "X");
                }
                path.push((0,guard_pos.1));
                break;
            }
        } else if guard_dir == (1,0) {
            let mut los = horiz.iter().map({
                |s| s.chars().nth(guard_pos.1).unwrap()
            }).collect::<String>()[guard_pos.0+1..].chars().collect::<String>();
            let mut next = los.chars().position(|c| c == '#');
            if next.is_some(){
                // change each character in los to be an X except for the last one
                guard_pos = (guard_pos.0 + (next.unwrap()), guard_pos.1);
                guard_dir = (0,-1);
                // println!("guard_pos: {:?}", guard_pos);
                for i in old_pos.0..guard_pos.0 {
                    horiz[i].replace_range(old_pos.1..guard_pos.1+1, "X");
                }
            }else{
                // we would walk off the board
                for i in guard_pos.0..horiz.len() {
                    horiz[i].replace_range(guard_pos.1..old_pos.1+1, "X");
                }
                path.push((horiz.len()-1,guard_pos.1));
                break;
            }
        } else if guard_dir == (0,-1) {
            let mut los = horiz[guard_pos.0][..guard_pos.1].chars();
            let mut next = los.rev().position(|c| c == '#');
            if next.is_some(){
                guard_pos = (guard_pos.0, guard_pos.1 - next.unwrap());
                guard_dir = (-1,0);
                // println!("guard_pos: {:?}", guard_pos);
                horiz[old_pos.0].replace_range(guard_pos.1..old_pos.1+1, &"X".repeat(next.unwrap()+1));
            }else{
                horiz[old_pos.0].replace_range(0..old_pos.1, &"X".repeat(old_pos.1));
                path.push((guard_pos.0,0));
                break;
            }
        } else if guard_dir == (0,1) {
            let mut los = horiz[guard_pos.0][guard_pos.1+1..].chars();
            // println!("los: {:?}", los);
            let mut next = los.position(|c| c == '#');
            // println!("next: {:?}", next);
            if next.is_some(){
                guard_pos = (guard_pos.0, guard_pos.1 + (next.unwrap()));
                guard_dir = (1,0);
                // println!("guard_pos: {:?}", guard_pos);
                horiz[old_pos.0].replace_range(old_pos.1..guard_pos.1, &"X".repeat(next.unwrap()));
            }else{
                // we would walk off the board
                let len = horiz[0].len();
                horiz[old_pos.0].replace_range(old_pos.1..len, &"X".repeat(len-old_pos.1));
                path.push((guard_pos.0,len-1));
                break;
            }
        }
        if (path.contains(&(guard_pos.0,guard_pos.1))){
            if let Some(&op) = path.last() {
                if op != guard_pos {
                    // Don't record when we stand in the same place but just trun
                    trap = true;
                    // println!("trap");
                    break;
                }
            }
        }
    }
    return (horiz, path, trap);
}


fn main(){
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);

    let mut guard_pos = (0,0);
    let mut guard_dir: (isize, isize) = (-1,0);
    let mut horiz: Vec<String> = Vec::new();
    for line in buffered.lines() {
        horiz.push(line.unwrap());
        let pos = horiz[horiz.len()-1].find("^");
        if pos.is_some() {
            // println!("{:?}", horiz);
            // println!("guard_dir: {:?}", guard_dir);
            guard_pos = (horiz.len()-1, pos.unwrap());
            horiz[guard_pos.0] = horiz[guard_pos.0].replace("^", "X");
            println!("guard_pos: {:?}", guard_pos);
        }
    }
    println!("{:?}", horiz);
    // let mut vert: Vec<String> = Vec::new();
    // for (i, line) in horiz.iter().enumerate() {
    //     for (j, ch) in line.chars().enumerate() {
    //         if vert.len() <= j {
    //             vert.push(String::new());
    //         }
    //         vert[j].push(ch);
    //     }
    // }
    // println!("vert:");
    // println!("{:?}", vert);

    let (res_map, path, valid) = check_layout(horiz.clone(), guard_pos);
    println!("{:?}", res_map);
    println!("sum: {}", res_map.iter().map(|s| s.chars().filter(|c| *c == 'X').count()).sum::<usize>());
    println!("path: {:?}", path);
    let mut prev_pos = path[0];
    let mut sum = 0;
    let mut valid_pos: HashSet<(usize, usize)> = HashSet::new();
    for pos in path.iter().skip(1) {
        // println!("prev_pos: {:?}", prev_pos);
        // println!("pos: {:?}", pos);
        if prev_pos.0 == pos.0 {
            // we are moving horizontally
            let range = if prev_pos.1 < pos.1 {
                prev_pos.1+1..pos.1+1
            } else {
                pos.1..prev_pos.1
            };
            println!("range: {:?}", range);
            for j in range {
                let mut new_map = horiz.clone();
                new_map[pos.0].replace_range(j..j+1, "#");
                // println!("Trying {},{}", pos.0, j);
                let(_,_,trap) = check_layout(new_map, guard_pos);
                if trap {
                    valid_pos.insert((pos.0,j));
                }
            }
        } else {
            // We are moving vertically
            let range = if prev_pos.0 < pos.0 {
                prev_pos.0+1..pos.0+1
            } else {
                pos.0..prev_pos.0
            };
            println!("range: {:?}", range);
            for j in range {
                let mut new_map = horiz.clone();
                new_map[j].replace_range(pos.1..pos.1+1, "#");
                // println!("Trying {},{}", j, pos.1);
                let(_,_,trap) = check_layout(new_map, guard_pos);
                if trap {
                    valid_pos.insert((j,pos.1));
                }
            }
        }
        prev_pos = pos.clone();
    }
    println!("sum: {}", valid_pos.len());
    
}