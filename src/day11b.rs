use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day11.txt");

fn main() {
    let mut memo = init_memo(75);
    let result: u128 = INPUT.split_whitespace().map(|x|calculate_stones(x.parse::<usize>().unwrap(), 75, &mut memo)).sum();

    println!("{result}")
}
fn calculate_stones(value: usize, step: usize, memory: &mut HashMap<usize, HashMap<usize,usize>>)->u128{
    if step == 0{
        return 1
    }
    if let Some(cached_result) = memory.get(&step).and_then(|m| m.get(&value)){
        return *cached_result as u128
    }
    let mut res:u128 = 0;
    if value == 0{
        res += calculate_stones(1, step-1, memory);
    }
    else if let Some((a,b)) = has_even_number_of_digits(&value){
        res += calculate_stones(a, step-1, memory) + calculate_stones(b, step-1, memory);
    }
    else { 
        res += calculate_stones(value*2024, step -1, memory);
    }
    memory.entry(step).or_insert_with(HashMap::new).insert(value, res as usize);
    res
    
}
fn init_memo(steps: usize) -> HashMap<usize, HashMap<usize, usize>> {
    (1..=steps).map(|i| (i, HashMap::new())).collect()
}

fn has_even_number_of_digits(n: &usize)-> Option<(usize, usize)>{
    let digit_string = n.to_string();
    if digit_string.len() % 2==0{
        let mid = digit_string.len()/2;
        let (a, b) = digit_string.split_at(mid);
        return Some((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
    }

    None
}