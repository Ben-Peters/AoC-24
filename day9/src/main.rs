use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Enumerate;
use std::path::Path;

fn printfilesys(files: &Vec<(usize,u32)>, spaces: &Vec<u32>){
    for (&(file_id, file_len), &space_len) in files.iter().zip(spaces.iter()) {
        for i in 0..file_len {
            print!("{}", file_id);
        }
        for i in 0..space_len {
            print!{"."};
        }
    }
    println!("");
}
fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: String = String::new();
    for line in buffered.lines() {
        data = line.unwrap();
    }
    let mut files = data.chars().enumerate()
        .filter(|(i, _)| i%2 == 0)
        .map(|(_, c)| c.to_digit(10).unwrap()).enumerate()
        .collect::<Vec<(usize,u32)>>();
    let mut spaces = data.chars().enumerate()
        .filter(|(i, _)| i%2 == 1)
        .map(|(_, c)| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut files_rev = files.iter().rev();
    let mut sum: u64 = 0;
    let mut pos = 0;
    let (mut last_id, mut last_len) = *files_rev.next().unwrap();
    let mut rem_file = last_len;
    let mut filesystem: String = String::new();
    for (&(file_id, file_len), &space) in files.iter().zip(spaces.iter()) {
        // check if files_rev is pointing an an element before the current file
        // if it is, then we are done
        if last_id <= file_id {
            for i in 0..rem_file {
                sum += ((last_id as u32)*(i+pos))as u64;
                print!("{} ", last_id);
                filesystem.push_str(&last_id.to_string());
            }
            break;
        }
        println!("{}: {}", file_len, space);
        // Add up the current file
        for i in 0..file_len{
            sum += ((file_id as u32)*(i+pos)) as u64;
            print!("{} ", file_id);
            filesystem.push_str(&file_id.to_string());
        }
        // Fill the space from the back of the file array
        pos += file_len;
        for i in 0..space {
            sum += (last_id as u32*(i+pos)) as u64;
            rem_file -= 1;
            print!("{} ", last_id);
            filesystem.push_str(&last_id.to_string());
            if rem_file == 0 {
                (last_id, last_len) = *files_rev.next().unwrap();
                rem_file = last_len;
            }
        }
        pos += space;
    }
    println!("filesystem: {}", filesystem);
    println!("sum: {}", sum);
    files = data.chars().enumerate()
        .filter(|(i, _)| i%2 == 0)
        .map(|(_, c)| c.to_digit(10).unwrap()).enumerate()
        .collect::<Vec<(usize,u32)>>();
    spaces = data.chars().enumerate()
        .filter(|(i, _)| i%2 == 1)
        .map(|(_, c)| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    spaces.push(0);
    files_rev = files.iter().rev();
    let mut filesys: Vec<(usize,u32)> = files.clone();
    for &(last_id, last_len) in files_rev {
        // println!("Filesys: {:?}", filesys);
        // println!("Spaces: {:?}", spaces);
        // printfilesys(&filesys, &spaces);
        // println!("{}: {}", last_id, last_len);
        if let Some((i,_)) = spaces.iter().enumerate().find(|&(_,&len)| len >= last_len){
            let cur_idx = filesys.iter().position(|&(id,_)| id == last_id).unwrap();
            // If we found a spot this can fit move it there and shrink the remaining space
            if i >= cur_idx {
                // the spot we found is after the current location so just leave it
                continue;
            }
            filesys.insert(i+1, (last_id, last_len));
            filesys.remove(cur_idx+1);
            spaces[i] -= last_len;
            spaces.insert(i,0);
            spaces[cur_idx] += last_len + spaces[cur_idx+1];
            spaces.remove(cur_idx+1);
        } else {
            // We couldn't find a space big enough so igrnore and move cur_idx
        }
    }
    sum = 0;
    pos = 0;
    for (&(file_id, file_len), &space_len) in filesys.iter().zip(spaces.iter()) {
        for i in 0..file_len {
            sum += ((file_id as u32)*(i+pos)) as u64;
        }
        pos += file_len;
        pos += space_len;
        for i in 0..space_len {
        }
    }
    println!("");
    printfilesys(&filesys, &spaces);
    println!("sum: {}", sum);
}
