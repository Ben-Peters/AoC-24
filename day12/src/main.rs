use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_reigon(map: &mut Vec<String>, c: char, pos: (i32,i32), corners: &mut Vec<((i32, i32), (i32,i32))>) -> (i32, i32){
    let mut area = 0;
    let mut perim = 0;
    for (i,j) in [(0,1), (0,-1), (1,0), (-1,0)].iter() {
        let new_i = i+pos.0;
        let new_j = j+pos.1;
        if let Some(new_line) = map.get(new_i as usize) {
            if let Some(new_c) = new_line.chars().nth(new_j as usize) {
                if new_c == c {
                    // Add this new letter to the area
                    map[new_i as usize].replace_range(new_j as usize..new_j as usize+1, "#");
                    let (new_area, new_perim) = find_reigon(map, c, (new_i, new_j), corners);
                    area += new_area + 1;
                    perim += new_perim;
                } 
                else if new_c == '#' {
                    // This is a space with the same letter we've seen before so it doesn't add to the perimeter
                }
                else {
                    // This is a space with a different letter so it adds to the perimeter
                    perim += 1;
                    corners.push(((new_i, new_j),(-*j,*i)));
                }
            }
        }
    }
    return (area, perim);
}

fn count_sides(c: Vec<((i32, i32), (i32,i32))>)-> i32{
    let mut sides = 0;
    //println!("{:?}", c);
    let mut next: (i32, i32);
    let mut corners_mut = c.clone();
    while let Some((corner,dir)) = corners_mut.pop(){
        //println!("{:?}", corners_mut);
        next = (corner.0+dir.0, corner.1+dir.1);
        if corners_mut.contains(&((corner.0+dir.0, corner.1+dir.1),dir)) || 
            corners_mut.contains(&((corner.0-dir.0, corner.1-dir.1),dir)){
            while corners_mut.contains(&(next,dir)){
                // Skip the remaining edges on this side
                corners_mut.remove(corners_mut.iter().position(|&pos| pos == (next,dir)).unwrap());
                next = (next.0+dir.0, next.1+dir.1);
            }
            next = (corner.0-dir.0, corner.1-dir.1);
            while corners_mut.contains(&(next,dir)){
                // Have to look in both directions to fully remove the side
                corners_mut.remove(corners_mut.iter().position(|&pos| pos == (next,dir)).unwrap());
                next = (next.0-dir.0, next.1-dir.1);
            }
        }
        sides += 1;
    }
    return sides;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<String> = Vec::new();
    for line in buffered.lines() {
        data.push(line.unwrap());
    }
    for row in data.iter_mut() {
        *row = format!("{}{}{}", ".", row, ".");
    }
    let pad_row = ".".repeat(data[0].len());
    data.insert(0, pad_row.clone());
    data.push(pad_row);

    data.iter().for_each(|r| println!("{}", r));
    let mut sum1 = 0;
    let mut num_sides = 0;
    for i in 1..data.len()-1 {
        for j in 1..data[0].len()-1 {
            let c = data[i].chars().nth(j).unwrap();
            if c != '.' {
                let mut corners: Vec<((i32, i32), (i32,i32))> = Vec::new();
                data[i].replace_range(j..j+1, "#");
                let (area, perim) = find_reigon(&mut data, c, (i as i32, j as i32), &mut corners);
                println!("{}: {} {}", c, area+1, perim);
                sum1 += (area+1)*perim;

                // count the number of sides of the current region
                let sides = count_sides(corners);
                num_sides += sides * (area+1);
                println!("{}: {}", c, sides);
                //data.iter().for_each(|r| println!("{}", r));
                data = data.iter().map(|r| r.replace("#", ".")).collect::<Vec<String>>();
            }
        }
    }
    println!("sum1: {:?}", sum1);
    println!("num_sides: {:?}", num_sides);

}

