use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn can_form(towels: &Vec<String>, tgt_pat: &str, memo: &mut HashMap<String, i64>) -> i64{
    let mut num_possible: i64 = 0;
    if memo.contains_key(tgt_pat){
        num_possible =  *memo.get(tgt_pat).unwrap();
        //println!("{} can be formed {} ways", tgt_pat, num_possible);
        return num_possible;
    }
    for towel in towels.iter(){
        if towel == tgt_pat{
            // If this matches the rest of the pattern add another way to form this pattern
            num_possible += 1;
        }
        if towel.len() < tgt_pat.len(){
            if tgt_pat.starts_with(towel){
                let new_tgt = &tgt_pat[towel.len()..];
                //println!("{} starts with {}", tgt_pat, towel);
                num_possible += can_form(towels, new_tgt, memo);
            }
        }
    }
    //println!("{} can be formed {} ways", tgt_pat, num_possible);
    memo.insert(tgt_pat.to_string(), num_possible);
    return num_possible;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut towels: Vec<String> = Vec::new();
    let mut tgt_pats: Vec<String> = Vec::new();
    for line in buffered.lines(){
        let l = line.unwrap();
        if l.contains(","){
            l.split(", ").for_each(|x| towels.push(x.to_string()));
        }
        else if l.len() > 0{
            tgt_pats.push(l);
        }
    }

    //println!("{:?}", towels);
    //println!("{:?}", tgt_pats);

    let mut num_possible =0;
    let mut total_possible = 0;
    // Look at longer towels first
    towels.sort_by(|a,b| b.len().cmp(&a.len()));
    // Check if each pattern can be formed from the toweles
    let mut memo: HashMap<String, i64> = HashMap::new();
    for pat in tgt_pats.iter(){
        //println!("Checking {}", pat);
        let c = can_form(&towels, pat, &mut memo);
        if c > 0{
            num_possible += 1;
            total_possible += c;
        }
    }
    println!("Part 1: {}", num_possible);
    println!("Part 2: {}", total_possible);
}
