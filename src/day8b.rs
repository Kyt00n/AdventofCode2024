use std::collections::{HashMap, HashSet};

const INPUT: &str= include_str!("../inputs/day8.txt");

fn main(){
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut antenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut counter: HashSet<(usize, usize)> = HashSet::new();

    for line in INPUT.lines(){
        arr.push(line.chars().collect());
    }

    for row in 0..arr.len(){
        for col in 0..arr[0].len(){
            if arr[row][col] != '.'{
                antenas.entry(arr[row][col]).or_insert(Vec::new()).push((row, col))
            }
        }
    }
    for (_antena, cords) in antenas{
        
        if cords.len() > 1 {
            for &cord in &cords {
                counter.insert(cord);
            }
        }
        for i in 0..cords.len(){
            for j in 0..cords.len(){
                if i == j{
                    continue
                }
                let r = cords[i].0 as isize;
                let c = cords[i].1 as isize;
                let r2 = cords[j].0 as isize;
                let c2 = cords[j].1 as isize;

                let row = r2 + (r2 - r);
                let column = c2 + (c2 - c);
                if row >=0 && column >= 0 && row < arr.len() as isize && column  < arr[0].len() as isize{
                    counter.insert((row as usize, column as usize));
                    arr[row as usize][column as usize] = '#';
                }
                calculate_next(r, c, r2, c2, &mut counter,  &mut arr);               
            }
        }
    }
    
    println!("{}", counter.len());
    
    
}
fn calculate_next(r: isize, c: isize, r2:isize, c2: isize, cout: &mut HashSet<(usize, usize)>, arr: &mut Vec<Vec<char>>){
    let row = r2 + (r2 - r);
    let column = c2 + (c2 - c);
    if row >=0 && column >= 0 && row < arr.len() as isize && column  < arr[0].len() as isize{
        cout.insert((row as usize, column as usize));
        cout.insert((r as usize, c as usize));
        cout.insert((r2 as usize, c2 as usize));
        arr[row as usize][column as usize] = '#';
        calculate_next(r2, c2, row, column, cout, arr)
    }
}
