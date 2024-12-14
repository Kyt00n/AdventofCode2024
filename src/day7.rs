use std::collections::HashMap;


const INPUT: &str= include_str!("../inputs/day7.txt");

fn main(){
    let mut potential = 0;
    let mut map: HashMap<u64, Vec<usize>> = HashMap::new();

    for line in INPUT.lines(){
        let ab= line.split(':').collect::<Vec<&str>>();
        let a = ab[0].parse().unwrap();
        let b:Vec<usize> = ab[1].trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        map.insert(a, b);
    }
    for (answer, arr) in map{
        
        if let Some(result) = recursive_search(answer, arr[0] as u64, 1, &arr){
            potential+=result;   
        }
    }
    println!("{}", potential)
}

fn recursive_search(ans: u64, val: u64, i:usize, arr: &Vec<usize>)-> Option<u64>{
    if ans == val && i == arr.len(){
        return Some(val);
    }
    if i+1 > arr.len(){
        return None;
    }
    if let Some(result) = recursive_search(ans, val+arr[i] as u64, i+1, arr){
        return Some(result);
    }
    if let Some(result) = recursive_search(ans, val*arr[i] as u64, i+1, arr){
        return Some(result);
    }
    
    None
    
}
