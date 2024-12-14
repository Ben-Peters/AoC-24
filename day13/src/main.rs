use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use nalgebra::{Vector2,Matrix2};

fn win_game(a: (f64,f64), b: (f64,f64), prize: (f64,f64)) -> i64 {
    let matrix = Matrix2::new(a.0, b.0,
                                                                a.1, b.1);
    let inv_matrix = matrix.try_inverse().unwrap();
    let prize_v = Vector2::new(prize.0, prize.1);
    let buttons_v = inv_matrix*prize_v;
    if (buttons_v[0].fract() < 1e-3 || buttons_v[0].fract() > 0.999) && 
        (buttons_v[1].fract() < 1e-3 || buttons_v[1].fract() > 0.999) {
        return ((buttons_v[0]+0.1).trunc() as i64 * 3) + (buttons_v[1]+0.1).trunc() as i64;
    }
    else {
        println!("No win: {:?}", buttons_v);
        return 0;
    }
}

fn main() {
    let path = Path::new("./data/input.txt");
    let input = File::open(&path).unwrap();
    let buffered = io::BufReader::new(input);
    let mut data: Vec<((f64,f64),(f64,f64),(f64,f64))> = Vec::new();
    let mut a: (f64,f64) = (0.0,0.0);
    let mut b: (f64,f64) = (0.0,0.0);
    let mut prize: (f64,f64);
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.contains("Button A:"){
            println!("{}", l.split_whitespace().nth(2).unwrap().replace("X","").replace(",",""));
            let x = l.split_whitespace().nth(2).unwrap().replace("X","").replace(",","").parse::<f64>().unwrap();
            let y = l.split_whitespace().last().unwrap().replace("Y","").replace(",","").parse::<f64>().unwrap();
            a = (x,y);
        } else if l.contains("Button B:"){
            // Format is Button B: X+27, Y+71
            // get the x and y offset preserving the sign and store it as a tuple
            let x = l.split_whitespace().nth(2).unwrap().replace("X","").replace(",","").parse::<f64>().unwrap();
            let y = l.split_whitespace().last().unwrap().replace("Y","").replace(",","").parse::<f64>().unwrap();
            b = (x,y);
        } else if l.contains("Prize:"){
            // Format is Prize: X=18641, Y=10279
            // Get the x and y coordinates and store that tuple
            let x = l.split_whitespace().nth(1).unwrap().split("=").last().unwrap().replace(",","").parse::<f64>().unwrap();
            let y = l.split_whitespace().last().unwrap().split("=").last().unwrap().parse::<f64>().unwrap();
            prize = (x,y);
            data.push((a,b,prize));
        }
    }
    let sum1: i64 = data.iter().map(|(a,b,prize)| win_game(*a,*b,*prize)).sum();
    println!("Sum1: {}", sum1);

    data = data.iter().map(|(a,b,(x,y))| (*a,*b,(x+10000000000000.0, y+10000000000000.0))).collect();
    let sum2: i64 = data.iter().map(|(a,b,prize)| win_game(*a,*b,*prize)).sum();
    println!("Sum2: {}", sum2);
}