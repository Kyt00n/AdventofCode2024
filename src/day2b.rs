const INPUT: &str = include_str!("../inputs/day2.txt");
fn main() {
    let mut count = 0;
    for line in INPUT.lines(){
        let record:Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if check_if_safe_without_entry(&record){
            count += 1;
        }
    }
    println!("{}",count)

}
fn check_if_safe(record: &Vec<i32>) -> bool{
    let is_increasing = record.windows(2)
        .all(|w| w[0] < w[1]
            && (w[0]-w[1]).abs() >0
            && (w[0]-w[1]).abs() <4);
    let is_decreasing = record.windows(2)
        .all(|w| w[0] > w[1]&& (w[0]-w[1]).abs() >0
            && (w[0]-w[1]).abs() <4);
    is_decreasing || is_increasing
}
fn check_if_safe_without_entry(record: &Vec<i32>) -> bool{
    if check_if_safe(record){
        return true;
    }
    for i in 0..record.len(){
        let mut record_clone = record.clone();
        record_clone.remove(i);
        if(check_if_safe(&record_clone)){
            return true;
        };
    }
    false
}
