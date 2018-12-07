use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_lines_in_file(filename : &str) -> Result<Vec<String>,std::io::Error> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let lines = contents.to_string().split('\n').filter(|s| s.to_string() != "").map(|s| s.to_owned()).collect();
    Ok(lines)
}

fn parse_string_vector(v : &Vec<String>) -> Vec<i32> {
    v.iter().map(|s| match s.parse::<i32>() {
        Ok(n) => n,
        Err(e) => panic!("[{}] Invalid value in file", e)
    }).collect()
}

fn get_numbers(filename : &str) -> Vec<i32> {
    match read_lines_in_file(&filename) {
        Ok(contents) => parse_string_vector(&contents),
        Err(_) => panic!()
    }
}

fn sum_string_vector(filename : &str) -> i32 {
    get_numbers(&filename).iter().sum()
}

fn find_repeated_total_in_file(filename : &str) -> i32 {
    let numbers = get_numbers(&filename);    
    return find_repeated_total(numbers);    
}

fn find_repeated_total(numbers : Vec<i32>) -> i32 {
    let mut seen_before_set = HashSet::<i32>::new();
    seen_before_set.insert(0i32);
    
    let mut total = 0i32;

    while true {
        let mut iter = numbers.iter();
        while let Some(n) = iter.next() {
            total = total + n;
            if seen_before_set.contains(&total){
                return total;
            }
            seen_before_set.insert(total);
        }
    }
    panic!("Shouldn't get here")    
}

fn main() {
    println!("Day 1, pt1 - {}", sum_string_vector("input"));
    println!("Day 1, pt2 - {}", find_repeated_total_in_file("input"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_reads_line_in_file() {
        match read_lines_in_file("input.txt") {
            Ok(contents) => assert_eq!(contents.len(), 959),
            Err(_) => panic!()
        }
    }

    #[test]
    fn test_sums_contents_of_string_array() {
        assert_eq!(sum_string_vector("input.txt"), 2);
    }

    #[test]
    fn test_finds_second_frequency_occurance() {
        assert_eq!(find_repeated_total_in_file("input.txt"),1);
    }

    #[test]
    fn test_finds_second_frequency_occurance_sample() {
        assert_eq!(find_repeated_total(vec![7, 7, -2, -7, -4]),14);
    }
}