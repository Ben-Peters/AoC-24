use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;

fn count_overlapping_substring(text: &str, pattern: &str) -> usize  {
    let mut count = 0;
    let mut start = 0;
    let mut positions: Vec<usize> = Vec::new();

    while let Some(pos) = text[start..].find(pattern) {
        println!("{}: {}: {}",text, start, pos);
        count += 1;
        start += pos + 1; // Move start position to allow overlapping
        positions.push(pos+start);
    }

    count
}
fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);

    let mut data: Vec<String> = Vec::new();
    for line in buffered.lines() {
        data.push(line.unwrap());
    }
    let rows = data.len();
    let cols = data[0].len();
    let mut horiz: Vec<String> = data.clone();
    let mut vert: Vec<String> = Vec::new();
    let mut neg_diag: Vec<String> = Vec::new();
    let mut pos_diag: Vec<String> = Vec::new();
    for row in horiz.iter() {
        for (i, ch) in row.chars().enumerate() {
            if vert.len() <= i {
                vert.push(String::new());
            }
            vert[i].push(ch);
        }
    }
    for r in 0..rows+cols-1 {
        let mut diag = String::new();
        let mut i = min(rows-1, r) as usize;
        let mut j = max(0, r as isize-rows as isize+1) as usize;
        while i < rows && j < cols {
            diag.push(data[i].chars().nth(j).unwrap());
            if(i == 0){break;}
            i -= 1;
            j += 1;
        }
        pos_diag.push(diag);
    }

    for r in 0..rows+cols-1 {
        let mut diag = String::new();
        let mut i = max(0, r as isize-rows as isize+1) as usize;
        let mut j = max(0, rows as isize-1 - r as isize) as usize;
        while i < rows && j < cols {
            diag.push(data[i].chars().nth(j).unwrap());
            i += 1; 
            j += 1;
        }
        neg_diag.push(diag);
    }
    // println!("{:?}", horiz);
    // println!("{:?}", vert);
    println!("{:?}", pos_diag);
    println!("{:?}", neg_diag);

    let mut re = Regex::new(r"XMAS").unwrap();
    // let cnt_horiz = horiz.iter().map(|row| {
    //     re.captures_iter(&row).count()
    // }).sum::<usize>();
    let cnt_horiz = horiz.iter().map(|row| {
        // Count forward and then count backward
        re.captures_iter(&row).count() +
        re.captures_iter(&(row.chars().rev().collect::<String>())).count()
    }).sum::<usize>();
    let cnt_vert = vert.iter().map(|row| {
        re.captures_iter(&row).count() +
        re.captures_iter(&(row.chars().rev().collect::<String>())).count()
    }).sum::<usize>();
    let cnt_pos_diag = pos_diag.iter().map(|row| {
        re.captures_iter(&row).count() +
        re.captures_iter(&(row.chars().rev().collect::<String>())).count()
    }).sum::<usize>();
    let cnt_neg_diag = neg_diag.iter().map(|row| {
        re.captures_iter(&row).count() +
        re.captures_iter(&(row.chars().rev().collect::<String>())).count()
    }).sum::<usize>();

    println!("Horiz: {}", cnt_horiz);
    println!("Vert: {}", cnt_vert);
    println!("Pos Diag: {}", cnt_pos_diag);
    println!("Neg Diag: {}", cnt_neg_diag);
    println!("Total: {}", cnt_horiz + cnt_vert + cnt_pos_diag + cnt_neg_diag);

    re = Regex::new(r"MAS").unwrap();

    let mut pos_matches: Vec<(isize,isize)> = Vec::new();
    for (r, row) in pos_diag.iter().enumerate() {
        pos_matches.extend(
            re.find_iter(&row)
            .map(|m| {((r) as isize, (m.start()+1)as isize)})
        );
        pos_matches.extend(
            re.find_iter(&(row.chars().rev().collect::<String>()))
            .map(|m| {(r as isize, ((row.len()-m.start())-2) as isize)})
        );
    }
    let mut neg_matches: Vec<(isize,isize)> = Vec::new();
    for (r, row) in neg_diag.iter().enumerate() {
        println!("Row: {}", row);
        neg_matches.extend(
            re.find_iter(&row)
            .map(|m| {((r) as isize, (m.start()+1) as isize)}));
        neg_matches.extend(
            re.find_iter(&(row.chars().rev().collect::<String>()))
            .map(|m| {((r) as isize, ((row.len()-m.start())-2) as isize)})
        );
    }

    //convert positions in the diagonals to positions in the orriginal grid
    pos_matches = pos_matches.iter().map(|(i,j)| {
        let orig_i = (i - max(0, i - (rows as isize - 1)) - j);
        let orig_j = j + max(0, i - (rows as isize - 1));
        //println!("({}:{}) -> ({}:{})", i, j, orig_i,orig_j);
        (orig_i,orig_j)
    }).collect::<Vec<(isize,isize)>>();

    neg_matches = neg_matches.iter().map(|(i,j)| {
        let orig_i = j + max(0, i - (rows as isize - 1));
        let orig_j = max(0, (rows as isize - 1) -i) + j;
        println!("({}:{}) -> ({}:{})", i, j, orig_i,orig_j);
        (orig_i,orig_j)
    }).collect::<Vec<(isize,isize)>>();

    let mut sum = 0;
    for (i,j) in pos_matches.iter() {
        if neg_matches.contains(&(*i,*j)){
            println!("{}:{}",i,j);
            sum += 1;
        }
    }
    println!("Sum: {}", sum);

}
