const INPUT: &str= include_str!("../inputs/day5.txt");

fn main() {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut has_been_empty = false;
    let mut counter = 0;
    for line in INPUT.lines(){
        if line == ""{
            has_been_empty =true;
            continue;
        }
        if has_been_empty {
            updates.push(
                line.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect()
            )
        }
        else {
            rules.push(new_rule(line));
        }
    }
    for update in updates{
        if check_if_incorrect(&update, &rules){
            let mut update_clone = update.clone();
            counter += reorder_incorrect_updates(&mut update_clone, &rules);
        }
    }
    println!("{}", counter)
}
fn new_rule(line: &str) -> (i32, i32){
    let parts: Vec<&str> = line.split("|").collect();
    (
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap()
    )
}
fn check_if_incorrect(arr:&Vec<i32>, ruleset: &Vec<(i32, i32)>) -> bool{
    for rule in ruleset{
        if arr.contains(&rule.1){
            let index = arr.iter().position(|n| n== &rule.1).unwrap();
            for i in index..arr.len(){
                if arr[i] == rule.0{
                    return true
                }
            }
        }
    }
    false
}
fn reorder_incorrect_updates(update: &mut Vec<i32>, ruleset:&Vec<(i32, i32)>) -> i32{
    for rule in ruleset{
        if update.contains(&rule.1) {
            let index = update.iter().position(|n| n == &rule.1).unwrap();
            for i in index..update.len() {
                if update[i] == rule.0 {
                    let value = update.remove(i);
                    update.insert(index, value);
                    
                    reorder_incorrect_updates(update, ruleset);
                }
            }
        }
    }
    find_middle(update)
}

fn find_middle(arr:&Vec<i32>) -> i32{
    if arr.len() %2 ==0{
        return arr[arr.len()-1/2] + arr[arr.len()+1/2]
    }
    arr[(arr.len()-1)/2]
}