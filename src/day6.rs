const INPUT: &str= include_str!("../inputs/day6.txt");

struct Maze{
    grid: Vec<Vec<char>>
}
impl Maze{
    fn new(input: &str)->Self{
        let grid = input.lines()
            .map(|line| line.trim()
                .chars()
                .collect())
            .collect();
        Maze{ grid }
    }
    fn print_maze(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
    
    fn find_start(&self) -> Option<(usize, usize, char)>{
        for (i, row) in self.grid.iter().enumerate(){
            if let Some(j) = row.iter().position(|&c| {
                c== '^' ||
                c=='>' || 
                c=='<' || 
                c=='v'
            }){
                return Some((i, j, self.grid[i][j]));
            }
        }
        unreachable!()
    }
    fn solve(&mut self) ->i32{
        let (i, j, direction) = self.find_start().unwrap();
        self.walk(i, j, direction);
        
        self.count_xs() +1
    }
    fn count_xs(&self) -> i32 {
        let mut counter = 0;
        for row in &self.grid {
            counter += row.iter().filter(|&&c| c == 'X').count();
        }
        counter as i32
    }
    fn walk(&mut self, i: usize, j:usize, direction:char) -> Option<()>{
        // println!("Walking to ({}, {}) in direction '{}'", i, j, direction);
        let mut stack = vec![(i, j, direction)];
        while let Some((i,j, direction)) = stack.pop() {
            match direction {
                '^'=> {
                    if let Some(row) = self.grid.get(i.checked_sub(1)?) {
                        if row[j] == '#' {
                            stack.push((i,j,'>'));
                        } else {
                            self.grid[i][j] = 'X';
                            stack.push((i - 1, j, direction));
                        }
                    }
                }
                'v' => {
                    if let Some(row) = self.grid.get(i.checked_add(1)?) {
                        if row[j] == '#' {
                            
                            stack.push((i, j, '<'));
                        } else {
                            self.grid[i][j] = 'X';
                            stack.push((i + 1, j, direction));
                        }
                    }
                }
                '>' => {
                    if let Some(col) = self.grid[i].get(j.checked_add(1)?){
                        if *col == '#' {
                            
                            stack.push((i, j, 'v'));
                        } else {
                            self.grid[i][j] = 'X';
                            stack.push((i, j +1, direction));
                        }
                    }
                }
                '<' => if let Some(col) = self.grid[i].get(j.checked_sub(1)?){
                    if *col == '#' {
                        stack.push((i, j, '^'));
                    } else {
                        self.grid[i][j] = 'X';
                        stack.push((i, j -1, direction));
                    }
                }
                _=>{}
            }
        }
        
        Some(())
    }
    
}

fn main() {
    let mut maze = Maze::new(INPUT);
    let start = maze.solve();
    println!("{:?}", start);
    maze.print_maze();
}