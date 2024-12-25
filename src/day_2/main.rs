use std::io;

fn main() {
    let input = read_input();

    println!("safe readings: {}", count_safe(&input, test_safe));
    println!("safe with dampner: {}", count_safe(&input, test_safe_with_dampner));
}

fn count_safe(readings: &Vec<Vec<i32>>, safetey_function: fn(&Vec<i32>) -> bool) -> i32{
    readings
    .iter()
    .map(|readings| {
        if safetey_function(readings) {
            1
        } else {
            0
        }
    })
    .sum()
}

fn read_input() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    stdin.lines().map(|l| -> Vec<i32> {
        let line = l.unwrap();
        let split = line.split_whitespace();
        split.map(|num_string| -> i32 {
            num_string.parse::<i32>().unwrap()
        }).collect()
    }).collect()
}

fn test_safe(readings: &Vec<i32>) -> bool  {
    let mut x = readings
    .iter()
    .zip(readings.iter().skip(1))
    .map(|(a, b)| {
         a - b
    })
    .peekable();

    let is_decreasing = *x.peek().unwrap() > 0;
    x.all(|diff| {
        if diff == 0
        || diff.abs() > 3
        || (is_decreasing && diff < 0)
        || (!is_decreasing && diff > 0) {
            false
        } else {
            true
        }
    })
}

// TODO optimize
fn test_safe_with_dampner(readings: &Vec<i32>) -> bool {
    if test_safe(readings) {
        return true;
    }
    for i in 0..readings.len() {
        let mut sub_readings = readings.clone();
        sub_readings.remove(i);
        if test_safe(&sub_readings) {
            return true;
        }
    }
    return false;
}