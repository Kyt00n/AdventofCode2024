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
            if char_array[i][j] == 'X'{
                result += search_for_xmas(&char_array, i, j);
            }
        }
    }
    println!("{}", result);
}

fn search_for_xmas(arr: &Vec<Vec<char>>, i: usize, j:usize) -> u32{
    let mut counter = 0;
    //forward
    if arr[i].len() > j+3{
        if arr[i][j+1] == 'M' {
            if arr[i][j+2] == 'A' {
                if arr[i][j+3] == 'S' {
                    counter +=1;
                }
            }
        }
    }
    
    //backward
    if j >= 3{
        if arr[i][j-1] == 'M' {
            if arr[i][j-2] == 'A' {
                if arr[i][j-3] == 'S' {
                    counter +=1;
                }
            }
        }
    }
    //up
    if arr.len() > i+3{
        if arr[i+1][j] == 'M' {
            if arr[i+2][j] == 'A' {
                if arr[i+3][j] == 'S' {
                    counter +=1;
                }
            }
        }
    }
    //down
    if i >= 3{
        if arr[i-1][j] == 'M' {
            if arr[i-2][j] == 'A' {
                if arr[i-3][j] == 'S' {
                    counter +=1;
                }
            }
        }
    }
    
    //diagonal
    if arr[i].len() > j+3 {
        if arr.len() > i+3{
            //right up
            if arr[i + 1][j + 1] == 'M' {
                if arr[i + 2][j + 2] == 'A' {
                    if arr[i + 3][j + 3] == 'S' {
                        counter += 1;
                    }
                }
            }
        }
        if i >= 3{
            //right down    
            if arr[i-1][j+1] == 'M' {
                if arr[i-2][j+2] == 'A' {
                    if arr[i-3][j+3] == 'S'{
                        counter +=1;
                    }
                }
            }
        }
    }
    if j >= 3{
        if arr.len() > i+3 {
            // left up
            if arr[i + 1][j - 1] == 'M' {
                if arr[i + 2][j - 2] == 'A' {
                    if arr[i + 3][j - 3] == 'S' {
                        counter += 1;
                    }
                }
            }
        }
        if i >= 3{
            //left down
            if arr[i-1][j-1] == 'M' {
                if arr[i-2][j-2] == 'A' {
                    if arr[i-3][j-3] == 'S'{
                        counter +=1;
                    }
                }
            }
        }
    }    
    counter
}
