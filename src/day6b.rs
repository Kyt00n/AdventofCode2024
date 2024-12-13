use std::collections::VecDeque;
//this one doesn't work, but I've still tried my best and learned some structs and impls, so I'm still saving that :)
//correct one is day6b2.rs 
const INPUT: &str= include_str!("../inputs/test.txt");

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
        let mut loop_queue: VecDeque<(usize, usize, char)> = VecDeque::new();
        while let Some((i,j, direction)) = stack.pop() {
            if loop_queue.len()==3{
                if let Some(loop_coords) = self.check_for_loop(&loop_queue) {
                    println!("found: {:?}", loop_coords);
                }
                loop_queue.pop_front();
            }
            match direction {
                '^'=> {
                    if let Some(row) = self.grid.get(i.checked_sub(1)?) {
                        if row[j] == '#' {
                            stack.push((i,j,'>'));
                            loop_queue.push_back((i,j,direction))
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
                            loop_queue.push_back((i,j,direction))
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
                            loop_queue.push_back((i,j,direction))
                        } else {
                            self.grid[i][j] = 'X';
                            stack.push((i, j +1, direction));
                        }
                    }
                }
                '<' => if let Some(col) = self.grid[i].get(j.checked_sub(1)?){
                    if *col == '#' {
                        stack.push((i, j, '^'));
                        loop_queue.push_back((i,j,direction))
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
    fn check_for_loop(&self,arr: &VecDeque<(usize, usize, char)>)->Option<(usize,usize)>{
        let mut potential_loop:(usize, usize) = (0,0); 
        let x1 = arr[0];
        let x3 = arr[2];
        //get coordinates
        match x3.2{
            'v' => {
                potential_loop = (x3.0, x1.1);
                
            }
            '>' => {
                potential_loop = (x3.1, x1.0);
            }
            '<' => {
                potential_loop = (x1.1, x3.0);
            }
            '^' => {
                potential_loop = (x1.0, x3.1);
            }
            _=> {}
        }
        if self.check_for_boundary_vertically((x1.0, x1.1), potential_loop) && self.check_for_boundary_horizontally((x3.0, x3.1), potential_loop){
            return Some(potential_loop)
        }
        None
    }
    fn check_for_boundary_vertically(&self, x1:(usize, usize), x2:(usize, usize))->bool{
        let (start_j, end_j) = if x1.1 < x2.1 { (x1.1, x2.1) } else { (x2.1, x1.1) };
        for j in start_j..=end_j {
            if self.grid[x1.0][j] == '#' {
                return false;
            }
        }
        true
    }
    fn check_for_boundary_horizontally(&self, x1:(usize, usize), x2:(usize, usize))->bool{
        let (start_j, end_j) = if x1.0 < x2.0 { (x1.0, x2.0) } else { (x2.0, x1.0) };
        for j in start_j..=end_j {
            if self.grid[j][x1.1] == '#' {
                return false;
            }
        }
        true
    }

}

fn main() {
    let mut maze = Maze::new(INPUT);
    let start = maze.solve();
    println!("{:?}", start);
    maze.print_maze();
}