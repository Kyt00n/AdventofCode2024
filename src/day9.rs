use std::collections::VecDeque;

const INPUT: &str= include_str!("../inputs/day9.txt");

fn main(){
    let mut result: Vec<usize> = Vec::new();
    for line in INPUT.lines(){
        result = line.chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
    }
    let mut id_count = 0;
    let mut disk_map:VecDeque<i32> = VecDeque::new();
    for (i,file) in result.iter().enumerate(){
        for _ in 0..*file{
            if i %2 == 0{
                disk_map.push_back(id_count);
            }
            else {
                disk_map.push_back(-1);
            }
        }
        if i %2 == 0{
            id_count+=1;
        }
    }
    loop {
        if let Some(index) = disk_map.iter().position(|x| *x == -1) {
            if let Some(value) = disk_map.pop_back() {
                if index < disk_map.len() {
                    disk_map[index] = value;
                }
            }
        } else {
            break;
        }
    }
    let mut count:i64 = 0;
    for (i, file) in disk_map.iter().enumerate(){
        count += (i as i32* file) as i64;
    }
    println!("{}", count)
    
}