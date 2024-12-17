use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
    direction: (i32, i32),
    history: HashSet<((i32,i32))>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(pos: (i32, i32), goal: (i32, i32)) -> i32 {
    (pos.0 - goal.0).abs() + (pos.1 - goal.1).abs()
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut walls: Vec<Vec<bool>> = Vec::new();
    let mut start: (i32,i32) = (0, 0);
    let mut end: (i32,i32) = (0, 0);
    for line in buffered.lines() {
        let l = line.unwrap();
        walls.push(l.chars().map(|c| c == '#').collect::<Vec<bool>>());
        if l.contains("S") {
            start = ((walls.len() - 1) as i32, l.find("S").unwrap() as i32);
        }
        if l.contains("E") {
            end = ((walls.len() - 1) as i32, l.find("E").unwrap() as i32);
        }
    }

    // Use dijkstra's algorithm to find the shortest path

    let mut distances: HashMap<((i32,i32),(i32,i32)),i32> = HashMap::new();
    distances.insert(((start.0,start.1),(0,1)), 0);
    distances.insert(((start.0,start.1),(0,-1)), 0);
    distances.insert(((start.0,start.1),(-1,0)), 0);
    distances.insert(((start.0,start.1),(1,0)), 0);

    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: start,
        direction: (0, 1),
        history: HashSet::new(),
    });
    
    let directions = [
        ((0, 1), 1),  // Move right
        ((1, 0), 1),  // Move down
        ((0, -1), 1), // Move left
        ((-1, 0), 1), // Move up
    ];

    let mut min_score = i32::MAX;

    let mut on_min_path: HashSet<((i32,i32))> = HashSet::new();

    while let Some(State { cost, position, direction , mut history}) = queue.pop() {
        history.insert(position);
        if position == end {
            println!("Found end with cost: {}", cost);
            min_score = cost.min(min_score);
            if min_score == cost {
                println!("Min Len Path found: {}", min_score);
                on_min_path.extend(history.iter());
            }
            continue;
        }

        // if cost > distances[position.0 as usize][position.1 as usize] {
        //     continue;
        // }
        // println!("Cost: {}", cost);
        // for i in 0..walls.len(){
        //     for j in 0..walls[i].len(){
        //          if (i as i32, j as i32) == position {
        //             print!("{}", if direction == (0, 1) { ">" } else if direction == (0, -1) { "<" } else if direction == (1, 0) { "v" } else { "^" });
        //          }else if history.contains(&(i as i32, j as i32)) {
        //             print!("O");
        //         } else if (i as i32, j as i32) == start {
        //             print!("S");
        //         } else if (i as i32, j as i32) == end {
        //             print!("E");
        //         } else if walls[i][j] {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("")
        // }
        // println!("----------------------------");

        for &(new_dir, move_cost) in &directions {
            let new_pos = (position.0 + new_dir.0, position.1 + new_dir.1);
            if !walls[new_pos.0 as usize][new_pos.1 as usize] {
                let turn_cost = if new_dir == direction { 0 } else if  new_dir == (-direction.0,-direction.1) { 2000 } else { 1000 };
                let new_cost = cost + move_cost + turn_cost;
                if new_cost < distances.get(&(new_pos, new_dir)).cloned().unwrap_or(i32::MAX) {
                    distances.insert((new_pos, new_dir),new_cost);
                    queue.push(State {
                        cost: new_cost,
                        position: new_pos,
                        direction: new_dir,
                        history: history.clone(),
                    });
                } else if new_cost == distances.get(&(new_pos, new_dir)).cloned().unwrap_or(i32::MAX) {
                    // keep looking, this might just be an alternate path
                    queue.push(State {
                        cost: new_cost,
                        position: new_pos,
                        direction: new_dir,
                        history: history.clone(),
                    });
                }
            } 
        }
    }

    for i in 0..walls.len(){
        for j in 0..walls[i].len(){
            if on_min_path.contains(&(i as i32, j as i32)) {
                print!("O");
            } else if (i as i32, j as i32) == start {
                print!("S");
            } else if (i as i32, j as i32) == end {
                print!("E");
            } else if walls[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("Min score: {}", min_score);
    println!("On min path: {:?}", on_min_path.len());
}