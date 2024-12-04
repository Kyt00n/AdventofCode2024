use fancy_regex::Regex;
const INPUT: &str= include_str!("../inputs/day3.txt");

fn main(){
    let mut result=0;
    let re_negative_matches = Regex::new(r"don't\(\)(?s:(?!do\(\)).)*?do\(\)").unwrap().replace_all(INPUT, "");
    
    let re = regex::Regex::new(r"(mul\(\d*,\d*\))").unwrap();
    let matches: Vec<&str> = re.find_iter(&re_negative_matches).map(|m| m.as_str()).collect();
    let re = regex::Regex::new(r"(\d*),(\d*)").unwrap();
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