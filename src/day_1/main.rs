use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut input = read_input();
    input.0.sort();
    input.1.sort();
    let distance = distance(&input.0, &input.1);
    println!("distance: {distance}");
    let similarity_score = similarity_score(&input.0, &input.1);
    println!("similarity_score: {similarity_score}");
}

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let stdin = io::stdin();
    stdin.lock().lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let a = split.next().unwrap().parse::<i64>().unwrap();
        let b: i64 = split.next().unwrap().parse::<i64>().unwrap();
        (a, b)
    }).unzip()
}

fn distance(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn similarity_score(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut b_freq_map: HashMap<i64, i64> = HashMap::new();
    for n in b {
       b_freq_map.insert(*n, 1 + b_freq_map.get(n).unwrap_or(&0));
    }
    a.iter().fold(0, |acc: i64, n| {
        acc + (n * b_freq_map.get(n).unwrap_or(&0))
    })
}