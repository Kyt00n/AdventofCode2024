use regex::Regex;
const INPUT: &str= include_str!("../inputs/day3.txt");

fn main(){
    let mut result=0;
    let re = Regex::new(r"(mul\(\d*,\d*\))").unwrap();
    let matches: Vec<&str> = re.find_iter(INPUT).map(|m| m.as_str()).collect();
    let re = Regex::new(r"(\d*),(\d*)").unwrap();
    for entry in matches{
        re.captures(entry).map(|n| {
            let(_, [a, b]) = n.extract();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            result+= a*b;
        });
    }
    println!("{}",result)
}