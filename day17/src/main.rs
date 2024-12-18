use core::{num, panic};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

enum Instruction{
    adv = 0,    // A / 2^Combo -> A
    bxl = 1,    // B XOR Literal -> B
    bst = 2,    // Cobmo % 8 -> B
    jnz = 3,    // jump if A != 0 to literal
    bxc = 4,    // B XOR C -> B
    out = 5,    // Combo % 8 -> output
    bdv = 6,    // A / 2^Combo -> B
    cdv = 7,    // A / 2^Combo -> C
}
use std::convert::From;
impl From<usize> for Instruction {
    fn from(item: usize) -> Self {
        match item {
            0 => Instruction::adv,
            1 => Instruction::bxl,
            2 => Instruction::bst,
            3 => Instruction::jnz,
            4 => Instruction::bxc,
            5 => Instruction::out,
            6 => Instruction::bdv,
            7 => Instruction::cdv,
            _ => panic!("Invalid instruction: {}", item),
        }
    }
}

fn combo_to_literal(combo: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    if combo <= 3{
        return combo;
    } else if combo == 4 {
        return reg_a;
    } else if combo == 5 {
        return reg_b;
    } else if combo == 6 {
        return reg_c;
    } else {
        panic!("Invalid combo: {}", combo);
    }
}

fn run_program(a: usize, b: usize, c: usize, program: Vec<usize>, short_circut: bool) -> Vec<usize>{
    let mut output = Vec::new();
    let mut i_ptr= 0;
    let mut reg_a = a;
    let mut reg_b = b;
    let mut reg_c = c;
    while i_ptr < program.len()-1 {
        let instruction: Instruction = Instruction::from(*(program.get(i_ptr).unwrap()));
        let operand = *program.get(i_ptr + 1).unwrap();
        match instruction {
             Instruction::adv=> {
                // A / 2^Combo -> A
                reg_a = reg_a / 2_usize.pow(combo_to_literal(operand, reg_a, reg_b, reg_c) as u32);
                i_ptr += 2;
            },
            Instruction::bxl => {
                // B XOR Literal -> B
                reg_b ^= operand;
                i_ptr += 2;
            },
            Instruction::bst => {
                // Cobmo % 8 -> B
                reg_b = combo_to_literal(operand, reg_a, reg_b, reg_c) % 8;
                i_ptr += 2;
            },
            Instruction::jnz => {
                // if A != 0 jump to literal
                if reg_a != 0 {
                    i_ptr = operand;
                } else {
                    i_ptr += 2;
                }
            },
            Instruction::bxc => {
                // B XOR C -> B
                reg_b ^= reg_c;
                i_ptr += 2;
            },
            Instruction::out => {
                // Combo % 8 -> output
                let out = combo_to_literal(operand, reg_a, reg_b, reg_c) % 8;
                if short_circut && (output.len() >= program.len() || out != program[output.len()]) {
                    // Short-circuit the program if the output doesn't match the program
                    return output;
                }
                output.push(out);
                i_ptr += 2;
            },
            Instruction::bdv => {
                // A / 2^Combo -> B
                reg_b = reg_a / 2_usize.pow(combo_to_literal(operand, reg_a, reg_b, reg_c) as u32);
                i_ptr += 2;
            },
            Instruction::cdv => {
                // A / 2^Combo -> C
                reg_c = reg_a / 2_usize.pow(combo_to_literal(operand, reg_a, reg_b, reg_c) as u32);
                i_ptr += 2;
            }
        }
    }
    return output;
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut program: Vec<usize> = Vec::new();
    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut i_ptr = 0;
    for line in buffered.lines(){
        let l = line.unwrap();
        if l.contains("Register A:"){
            reg_a = l.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        } else if l.contains("Register B:"){
            reg_b = l.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        } else if l.contains("Register C:"){
            reg_c = l.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        } else if l.contains("Program:"){
            l.split_whitespace().nth(1).unwrap().split(",").for_each(|s|{
                program.push(s.parse::<usize>().unwrap())});
        }
    }
    let mut output: Vec<usize> = run_program(reg_a, reg_b, reg_c, program.clone(), false);
    println!("Output: {:?}", output);
    reg_a = 10761;//1043738016928528;
    let mut min_seen = 1;

    let mut matching_lower_bits: HashSet<usize> = HashSet::new();
    matching_lower_bits.insert(0);
    let mut working_nums = HashSet::new();
    for num_matching in 0..program.len(){
        // find all patterns that match 1, then 2, then 3, etc.
        let prior_matches = matching_lower_bits.clone();
        matching_lower_bits.clear();
        for suffix in prior_matches{
            println!("Suffix: {:b}", suffix);
            for prefix in 0..1023{
                // generate all possible next patterns
                // check if the new pattern works with any of the lower patterns
                let new_num = prefix << num_matching*3 | suffix;
                output = run_program(new_num, reg_b, reg_c, program.clone(),true);
                if output.len() > num_matching {
                    //if !matching_lower_bits.contains(&(new_num  % (8 << (num_matching*3)))) {println!("Found a longer match: {:b}: {:?}\nLocking in {:b}", new_num, output, new_num  % (8 << (num_matching*3)));}
                    matching_lower_bits.insert(new_num  % (8 << (num_matching*3)));
                    if output == program {
                        println!("Found A: {}, {:?}", new_num, output);
                        working_nums.insert(new_num);
                    }
                }
            }
        }
    } 

    println!("Min woking number: {} : {:?}", working_nums.iter().min().unwrap(), run_program(*working_nums.iter().min().unwrap(), reg_b, reg_c, program.clone(),true));

    // for i in 0..100000{
    //     let output = run_program(i, reg_b, reg_c, program.clone(),true);
    //     if output.len() >= min_seen {
    //         min_seen = output.len();
    //         println!("Found A: {}, %len {}: {:?}", i, i%(1<<((output.len())*3)), output);
    //     }
    // }
    // println!("Output: {:?}", run_program(reg_a, reg_b, reg_c, program.clone(), false));
    // loop {
    //     if reg_a % 100000 == 0 {
    //         println!("Checking A (x100k): {}", reg_a/100000);
    //     }
    //     if run_program(reg_a, reg_b, reg_c, program.clone()) == program {
    //         println!("Found A: {}", reg_a);
    //         break;
    //     }
    //     reg_a += 1;
    // }
}
