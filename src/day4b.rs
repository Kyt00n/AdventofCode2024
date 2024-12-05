const INPUT: &str= include_str!("../inputs/day4.txt");

fn main() {
    let mut char_array: Vec<Vec<char>> = Vec::new();
    let mut result = 0;
    for line in INPUT.lines(){
        let  char_line_array:Vec<char> = line.trim()
            .chars()
            .collect();
        char_array.push(char_line_array)
    }
    for i in 0..char_array.len(){
        for j in 0..char_array[i].len(){
            if char_array[i][j] == 'A'{
                result += search_for_x_mas(&char_array, i, j);
            }
        }
    }
    println!("{}", result);
}
fn search_for_x_mas(arr: &Vec<Vec<char>>, i: usize, j:usize) -> u32{
    let mut counter = 0;
    
    let up_right = |c: char| -> bool {
        if arr[i].len() > j + 1 {
            if arr.len() > i + 1 {
                if arr[i + 1][j + 1] == c {
                    return true;
                }
            }
        }
        false
    };
    let up_left = |c: char| -> bool {
        if j >= 1 {
            if arr.len() > i + 1 {
                if arr[i + 1][j - 1] == c {
                    return true;
                }
            }
        }
        false
    };
    let down_right = |c: char| -> bool {
        if arr[i].len() > j + 1 {
            if i >= 1 {
                if arr[i - 1][j + 1] == c {
                    return true
                }
            }
        }
        false
    };
    let down_left = |c: char| -> bool {
        if j >= 1 {
            if i >= 1 {
                if arr[i - 1][j - 1] == c {
                    return true
                }
            }
        }
        false
    };
    if up_left('M') && down_right('S') {
        if up_right('M') && down_left('S'){
            println!("found X-MAS at {i} {j}");
            counter += 1;
        }
        else if up_right('S') && down_left('M'){
            println!("found X-MAS at {i} {j}");
            counter += 1;
        }
    }
    else if up_left('S') && down_right('M') {
        if up_right('M') && down_left('S'){
            println!("found X-MAS at {i} {j}");
            counter += 1;
        }
        else if up_right('S') && down_left('M'){
            println!("found X-MAS at {i} {j}");
            counter += 1;
        }
    }
    return counter;
}