const INPUT: &str = include_str!("../inputs/day2.txt");
fn main() {
    let mut count = 0;
    for line in INPUT.lines(){
        let record = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if check_i_safe(record){
            count += 1;
        }
    }
    println!("{}",count)
    
}
fn check_i_safe(record: Vec<i32>) -> bool{
    let is_increasing = record.windows(2)
        .all(|w| w[0] < w[1]
        && (w[0]-w[1]).abs() >0 
    && (w[0]-w[1]).abs() <4);
    let is_decreasing = record.windows(2)
        .all(|w| w[0] > w[1]&& (w[0]-w[1]).abs() >0
            && (w[0]-w[1]).abs() <4);
    is_decreasing || is_increasing
}
