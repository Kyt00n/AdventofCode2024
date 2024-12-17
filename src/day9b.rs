use std::collections::VecDeque;

const INPUT: &str= include_str!("../inputs/day9.txt");

fn main(){
    let mut result: Vec<usize> = Vec::new();
    for line in INPUT.lines(){
        result = line.chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
    }
    let mut id_count = 0;
    let mut disk_map:VecDeque<i32> = VecDeque::new();
    for (i,file) in result.iter().enumerate(){
        for _ in 0..*file{
            if i %2 == 0{
                disk_map.push_back(id_count);
            }
            else {
                disk_map.push_back(-1);
            }
        }
        if i %2 == 0{
            id_count+=1;
        }
    }
    
    let mut indexes_i: Vec<usize> = Vec::new();
    let mut indexes_j: Vec<usize> = Vec::new();
    let mut modified_map = disk_map.clone();
    
    for (i, file) in disk_map.iter().rev().enumerate(){
        if indexes_i.contains(&i) || *file ==-1{
            continue
        }
        let files = count_gaps(&disk_map, i, *file);
        for (j, point) in disk_map.iter().enumerate(){
            if *point != -1 || indexes_j.contains(&j){
                continue
            }
            if j > disk_map.len()-i{
                continue
            }
            
            
            // println!("i {i} file {file}");
            // if &current_file != file {
            //     // if indexes.len() != 0{
            //         current_file = *file;
                    
                    let gaps = count_gaps(&disk_map, j, -1);
                // println!("gaps {gaps}  files {files}  for {file} for arr{:?}", modified_map);
                    if gaps >=files && files !=0{
                        for k in 0..files as usize{
                            
                            // println!("{:?}", modified_map);
                            // println!("{} {} {}", modified_map.len(), i, k);
                            modified_map[j+k] = disk_map[disk_map.len()-i-k-1]; 
                            modified_map[disk_map.len()-i-k-1] = -1; 
                            // modified_map.remove(disk_map.len()-i-k-1);
                            // indexes_i.push(i+k);
                            indexes_j.push(j+k);
                        }
                        break
                    }
                    
                    
                    
                // }
            // }
            
            // indexes.push(i);
        }
        for k in 0..files as usize{
            // println!("blocked index {}", i+k);
            indexes_i.push(i+k);
        }
        
        // println!("{:?}", modified_map)
    }

    let mut count:i64 = 0;
    for (i, file) in modified_map.iter().enumerate(){
        if *file != -1 {
            count += (i as i32* file) as i64;
        }
    
    }
    println!("{}", count)

}
fn count_gaps(arr: &VecDeque<i32>, start_point: usize, v: i32) -> i32{
    if v == -1 {
        arr.iter()
            .skip(start_point)
            .take_while(|&&value| value == v)
            .count() as i32
    } else {
        arr.iter()
            .filter(|&&x| x == v)
            .count() as i32
    }
}