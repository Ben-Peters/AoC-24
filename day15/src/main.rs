use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn try_move(pos:(usize, usize), dir: char, walls: &HashSet<(usize,usize)>, boxes: &mut HashSet<(usize,usize)>, double_wide: bool) -> (bool, Vec<(usize,usize)>){
    let mut new_pos = pos;
    let mut new_box_pos: (i32,i32) = (new_pos.0 as i32, new_pos.1 as i32);
    let mut to_move: Vec<(usize,usize)> = Vec::new();
    match dir {
        '<' => {
            new_pos.1 -= 1;
            new_box_pos.1 -= 2;
        },
        '>' => {
            new_pos.1 += 1;
            new_box_pos.1 += 2;
        },
        '^' => {
            new_pos.0 -= 1;
            new_box_pos.0 -= 2;
        },
        'v' => {
            new_pos.0 += 1;
            new_box_pos.0 += 2;
        },
        _ => {
            println!("Invalid direction");
            return (false, to_move);
        }
    }
    if walls.contains(&(new_pos.0, new_pos.1)) || (walls.contains(&(new_pos.0, new_pos.1-1)) && double_wide) {
        // Running into a wall cannot move
        return (false, to_move);
    } else if boxes.contains(&(new_pos.0, new_pos.1)) || (boxes.contains(&(new_pos.0, new_pos.1-1)) && double_wide) {
        // There is a box, try and push it away
        let mut can_move: bool;
        let mut box_pos =new_pos;
        if !double_wide {
            (can_move, to_move) = try_move(new_pos, dir, walls, boxes, double_wide);
        } else {
            if boxes.contains(&(new_pos.0, new_pos.1-1)){
                // We hit the right edge of the box
                if dir == '<' {
                    // we are pushing left so only check next to current box
                    (can_move, to_move) = try_move((new_pos.0, new_pos.1-1), dir, walls, boxes, double_wide);
                } else {
                    let (can_move1, to_move1) = try_move(new_pos, dir, walls, boxes, double_wide);
                    let (can_move2, to_move2) = try_move((new_pos.0, new_pos.1-1), dir, walls, boxes, double_wide);
                    can_move = can_move1 && can_move2;
                    to_move.extend(to_move1);
                    to_move.extend(to_move2);
                        }
                new_box_pos.1 -= 1;
                box_pos.1 -= 1;

            }else{
                // We hit the left edge of the box
                if dir == '>' {
                    // we are moving right so we only need to check the spot next to the box
                    (can_move, to_move)  = try_move((new_pos.0,new_pos.1+1), dir, walls, boxes, double_wide);
                } else {
                    // Pushing up and down we need to check both spots
                    let (can_move1, to_move1) = try_move(new_pos, dir, walls, boxes, double_wide);
                    let (can_move2, to_move2) = try_move((new_pos.0, new_pos.1+1), dir, walls, boxes, double_wide);
                    can_move = can_move1 && can_move2;
                    to_move.extend(to_move1);
                    to_move.extend(to_move2);
                }
            }
        }
        if !can_move {
            to_move.clear();
            return (false, to_move);
        } else {
            // println!("{:?}", boxes);
            // println!("Moving box from {:?} to {:?}", box_pos, new_box_pos);

            to_move.push(box_pos);
            return (true, to_move);
        }
    } else {
        // Empty space so we can move here
        return (true, to_move);

    }

}
fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut walls: HashSet<(usize,usize)> = HashSet::new();
    let mut boxes: HashSet<(usize,usize)> = HashSet::new();
    let mut directions: Vec<char> = Vec::new();
    let mut robo_pos = (0,0);
    let mut map: Vec<Vec<char>> = Vec::new();
    for (i,line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        if l.contains("#"){
            map.push(Vec::new());
            l.char_indices().filter_map(|(j,c)| {
                if c == '#' {
                    Some((i, j))
                } else {
                    None
                }
            }).for_each(|(x,y)| {
                walls.insert((x,y));
            });
            l.char_indices().filter_map(|(j,c)| {
                if c == 'O' {
                    Some((i, j))
                } else {
                    None
                }
            }).for_each(|(x,y)| {
                boxes.insert((x,y));
            });
            if l.contains("@"){
                robo_pos = (i,l.find("@").unwrap());
            }
            l.chars().for_each(|c| map[i].push(c));
        } else if l.contains("<"){
            l.chars().for_each(|c| directions.push(c));
        }
    }
    for &dir in directions.iter(){
        let (can_move,to_move) = try_move(robo_pos, dir, &walls, &mut boxes, false);
        if can_move {
            let mut vect: (isize, isize) = (0,0);
            match dir {
                '<' => {
                    robo_pos.1 -= 1;
                    vect = (0,-1);
                },
                '>' => {
                    robo_pos.1 += 1;
                    vect = (0,1);
                },
                '^' => {
                    robo_pos.0 -= 1;
                    vect = (-1,0);
                },
                'v' => {
                    robo_pos.0 += 1;
                    vect = (1,0);
                },
                _ => {
                    println!("Invalid direction");
                }
            }
            to_move.iter().for_each(|(i,j)| {
                boxes.remove(&(*i,*j));
                boxes.insert(((*i as isize + vect.0) as usize, (*j as isize + vect.1) as usize));
            });
        }
    }
    let sum: usize = boxes.iter().map(|(i,j)| (i*100)+j).sum();
    println!("Sum: {}", sum);

    // Part 2
    let mut wide_map: Vec<Vec<char>> = Vec::new(); 
    let num_boxes = boxes.len();
    let num_walls = walls.len();
    boxes.clear();
    walls.clear();
    map.iter().enumerate().for_each(|(i,line)| {
        wide_map.push(Vec::new());
        line.iter().for_each(|&c|{
            match c {
                'O' => {
                    boxes.insert((i, wide_map[i].len()));
                    wide_map[i].push('[');
                    wide_map[i].push(']');
                },
                '#' => {
                    walls.insert((i, wide_map[i].len()));
                    wide_map[i].push('#');
                    wide_map[i].push('#');
                },
                '.' => {
                    wide_map[i].push('.');
                    wide_map[i].push('.');
                },
                '@' => {
                    robo_pos = (i, wide_map[i].len());
                    wide_map[i].push('@');
                    wide_map[i].push('.');
                },
                _ => {
                    print!("{}", c);
                }
            }
        })
    });
    // for i in 0..wide_map.len(){
    //     for j in 0..wide_map[i].len(){
    //         print!("{}", wide_map[i][j]);
    //     }
    //     println!("");
    // }
    // println!("Boxes: {:?}", boxes);
    // println!("Walls: {:?}", walls);
    for &dir in directions.iter(){
        let (can_move, to_move) = try_move(robo_pos, dir, &walls, &mut boxes, true);
        let old_boxes = boxes.clone();
        if can_move {
            let mut vect: (isize, isize) = (0,0);
            match dir {
                '<' => {
                    robo_pos.1 -= 1;
                    vect = (0,-1);
                },
                '>' => {
                    robo_pos.1 += 1;
                    vect = (0,1);
                },
                '^' => {
                    robo_pos.0 -= 1;
                    vect = (-1,0);
                },
                'v' => {
                    robo_pos.0 += 1;
                    vect = (1,0);
                },
                _ => {
                    println!("Invalid direction");
                }
            }
            // println!("{:?}, {:?}", to_move, vect);
            to_move.iter().for_each(|(i,j)| {
                // println!("{:?}", boxes);
                boxes.remove(&(*i,*j));
                boxes.insert(((*i as isize + vect.0) as usize, (*j as isize + vect.1) as usize));
                // println!("{:?}", boxes);
            });
            // for b in old_boxes.iter(){
            //     if !boxes.contains(b){
            //         println!("Box moved: {:?}", b);
            //     }
            // }
        }
        // println!("Direction: {}", dir);
        // println!("{}x {}", map.len(), wide_map[0].len());
        // for i in 0..map.len(){
        //     for j in 0..wide_map[0].len(){
        //         print!("{}", if (i,j) == robo_pos {
        //             "@"
        //         } 
        //         else if walls.contains(&(i,j)) || (j as i32 - 1 >= 0 && walls.contains(&(i,j-1))) {
        //             "#"
        //         } else if boxes.contains(&(i,j)) {
        //             "["
        //         } else if boxes.contains(&(i,j-1)) {
        //             "]"
        //         } else {
        //             "."
        //         });
        //     }
        //     println!("");
        // }
        // println!("----------------------------------");
        if boxes.len() != num_boxes || walls.len() != num_walls {
            println!("Boxes: {:?}", boxes);
        }
        assert_eq!(boxes.len(), num_boxes);
        assert_eq!(walls.len(), num_walls);
    }
    let sum2: usize = boxes.iter().map(|(i,j)| (i*100)+j).sum();
    println!("Sum: {}", sum2);

}
