const INPUT: &str = include_str!("../inputs/day11.txt");

fn main() {
    let mut stones: Vec<isize> = INPUT.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect();
    let steps = 25;
    for _step in 0..steps{
        let mut temp_stones: Vec<isize> = Vec::new();
        for stone in &stones{
            if *stone == 0{
                temp_stones.push(1);
            }
            else if let Some((a,b)) = has_even_number_of_digits(stone){
                temp_stones.push(a);
                temp_stones.push(b);
            }
            else{
                temp_stones.push(stone*2024)
            }
        }
        stones = temp_stones;
    }
    println!("{}",stones.len())
    
    
    
}
fn has_even_number_of_digits(n: &isize)-> Option<(isize, isize)>{
    let digit_string = n.to_string();
    if digit_string.len() % 2==0{
        let mid = digit_string.len()/2;
        let (a, b) = digit_string.split_at(mid);
        return Some((a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()));
    }
    
    None
}