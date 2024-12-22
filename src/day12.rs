use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day12.txt");

fn main() {
    let mut img: Vec<Vec<char>> = Vec::new();
    let mut count = 0;
    let mut visited_coords: HashSet<(usize, usize)> = HashSet::new();
    for line in INPUT.lines(){
        img.push(
            line.chars().collect()
        )
    }
    for row in 0..img.len(){
        for col in 0..img[0].len(){
            let target = img[row][col];
            if visited_coords.contains(&(row, col)) {
                continue;
            }
            let mut temp = vec![vec![false; img[0].len()]; img.len()];
            let result = flood_fill(&img, row, col, target, &mut temp);
            count += result.0 * result.1;
            for r in 0..img.len() {
                for c in 0..img[0].len() {
                    if temp[r][c] {
                        visited_coords.insert((r, c));
                    }
                }
            }
            println!("for {target}: {} {}", result.0, result.1)
        }
    }
    println!("{count}")
    
}
fn flood_fill(img: &Vec<Vec<char>>, r: usize, c:usize, target: char, visited: &mut Vec<Vec<bool>>) -> (usize, usize){
    let max_row=img.len();
    let max_col = img[0].len();
    let mut res = (0,0);
    if img[r][c] != target || visited[r][c]{
        return (0,0);
    }
    visited[r][c] = true;
    res.0 +=1;
    if r+1 < max_row{
        if img[r+1][c] !=target{
            res.1 += 1;
        }
        else { 
            let ab =flood_fill(&img, r+1, c, target, visited);
            res.0+=ab.0;
            res.1+=ab.1;
        }
    }
    else {
        res.1 += 1;
    }
    if let Some(previous_row) = r.checked_sub(1){
        if img[previous_row][c] !=target{
            res.1 += 1;
        }
        else {
            let ab =flood_fill(&img, previous_row, c, target, visited);
            res.0+=ab.0;
            res.1+=ab.1;
        }
    }
    else {
        res.1 += 1;
    }
    if c+1 < max_col{
        if img[r][c+1] !=target{
            res.1 += 1;
        }
        else {
            let ab =flood_fill(&img, r, c+1, target, visited);
            res.0+=ab.0;
            res.1+=ab.1;
        }
    }
    else {
        res.1 += 1;
    }
    if let Some(previous_col) = c.checked_sub(1){
        if img[r][previous_col] !=target{
            res.1 += 1;
        }
        else {
            let ab =flood_fill(&img, r, previous_col, target, visited);
            res.0+=ab.0;
            res.1+=ab.1;
        }
    }
    else {
        res.1 += 1;
    }
    return res
}