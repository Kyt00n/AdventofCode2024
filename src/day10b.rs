const INPUT: &str = include_str!("../inputs/day10.txt");

fn main() {
    let mut arr: Vec<Vec<usize>> = Vec::new();
    let mut starting_points: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    for (i,line) in INPUT.lines().enumerate(){
        let a: Vec<usize> = line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        
        
        for (index, &_value) in a.iter().enumerate().filter(|(_, &r)| r==0){
            starting_points.push((i, index))
        }
        arr.push(a)
    }
    
    for start in starting_points{
        count += check_path(&arr , start);
        
    }
    println!("{count}")
}

fn check_path(arr: &Vec<Vec<usize>>, coordinates: (usize, usize)) -> i32{
    
    let val = arr[coordinates.0][coordinates.1];
    if val == 9{
        println!("{} {} {}",coordinates.0, coordinates.1, val);
        return 1;
    }
    
    let mut count = 0;
    if coordinates.0+1 < arr.len(){
        if arr[coordinates.0+1][coordinates.1] == val+1{
            count += check_path(&arr, (coordinates.0+1, coordinates.1));
            
        }
    }
    if let Some(prev_row) = coordinates.0.checked_sub(1) {
        if arr[prev_row][coordinates.1] == val+1{
            count += check_path(&arr, (prev_row, coordinates.1));
            
        }
    }
    
    if coordinates.1+1 < arr[0].len(){
        if arr[coordinates.0][coordinates.1+1] == val+1{
            count += check_path(&arr, (coordinates.0, coordinates.1+1));
            
        }
    }
    
    if let Some(prev_col) = coordinates.1.checked_sub(1) {
        if arr[coordinates.0][prev_col] == val+1{
            count += check_path(&arr, (coordinates.0, prev_col));
            
        }
    }
        
    
    count
}

