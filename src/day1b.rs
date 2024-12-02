

const INPUT: &str = include_str!("../inputs/day1.txt");
fn main() {
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();
    let mut count = 0;
    for line in INPUT.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if numbers.len() == 2 {
            array1.push(numbers[0]);
            array2.push(numbers[1]);
        }
    }
    array1.sort();
    array2.sort();
    for number in array1 {
        count += number * array2.iter().filter(|&n| *n == number).count() as i32;
    }
    println!("{}", count);
    
}

