use std::collections::HashSet;

const INPUT: &str= include_str!("../inputs/day6.txt");

fn main(){
    let height = 130;
    let mut a:Vec<Vec<char>> = Vec::new();
    for line in INPUT.lines(){
        a.push(line.chars().collect())
    }
    let width = a[0].len();
    let mut position:(usize,usize) = (0,0);
    let dir = 0;
    let directions:Vec<(isize,isize)> = vec![(-1,0), (0,1), (1,0), (0,-1)];
    
    for row in 0..height{
        for col in 0..width{
            if a[row][col] == '^'{
                position = (row, col);
                a[row][col] = '.'
            }
        }
    }
    let start_me = position;
    let solve = |a: &mut Vec<Vec<char>>,start_me: (usize, usize), dir: usize| -> bool {
        let mut start_me = start_me;
        let mut dir = dir;
        let mut visited:HashSet<(usize, usize)> = HashSet::new();
        let mut turns = 0;
        loop {
            turns+=1;
            if turns > 10000{
                return true
            }
            visited.insert(start_me);
            let r2 = (start_me.0 as isize + directions[dir].0) as usize;
            let c2 = (start_me.1 as isize + directions[dir].1) as usize;
            if !(r2 < height && c2 < width) {
                return false
            }
            if a[r2][c2] == '.' {
                start_me = (r2, c2)
            } else {
                dir = (dir + 1) % 4;
            }
        }
    };

    
    let mut answer = 0;
    for row in 0..height{
        for col in 0..width{
            if a[row][col] == '.' && position != (row, col){
                a[row][col] = '#';
                if solve(&mut a,start_me, dir){
                    answer +=1;
                }
                a[row][col]='.';
            }
        }
    }
    
    println!("{}", answer);
}