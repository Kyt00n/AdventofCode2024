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
        // for pair in cords.windows(2){
        //     let x = pair[0].0.abs_diff(pair[1].0);
        //     let y = pair[0].1.abs_diff(pair[1].1);
        //     if pair[0].0>pair[1].0{
        //
        //     }
        //     counter.insert((pair[0].0-x, pair[0].1-y));
        //     counter.insert((pair[1].0+x, pair[1].1+y));
        //
        // }
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


            }
        }
    }
    // dbg!(&counter);
    println!("{}", counter.len());
    for row in &arr {
        println!("{:?}", row);
    }}