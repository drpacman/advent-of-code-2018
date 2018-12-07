use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_lines_in_file(filename : &str) -> Result<Vec<String>,std::io::Error> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let lines = contents.to_string()
                        .split('\n')
                        .filter(|s| s.to_string() != "")
                        .map(|s| s.to_owned())
                        .collect();
    Ok(lines)
}

fn calculate(entries : Vec<String>) -> i32 {
    let twos : i32 = entries.iter().filter(|s| has_two(s)).count() as i32;
    let threes : i32 = entries.iter().filter(|s| has_three(s)).count() as i32;
    return twos * threes;
}

fn has_two(s : &str) -> bool {
    return score(s).iter().find(|elem| *elem.1 == 2).is_some();
}

fn has_three(s : &str) -> bool {
    return score(s).iter().find(|elem| *elem.1 == 3).is_some();
}

fn score(s : &str) -> HashMap<char, i32> {
    let mut scores = HashMap::<char, i32>::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        let v = match scores.get(&c) {
            Some(n) => *n+1,
            None => 1
        };
        scores.insert(c, v);
    };
    return scores;
}

fn find_box(mut boxes : Vec<String>) -> Option<String> {
    boxes.sort();
    let mut it = boxes.iter();
    let mut first = it.next();
    let mut second = it.next();
    let mut res = None;
    while second.is_some() && !res.is_some()  {
        match (first, second) {
            (Some(s1), Some(s2)) => {
                res = matches(s1.clone(), s2.clone()) 
            },
            _ => ()
        }
        first = second;
        second = it.next();
    }
    return res;
}

fn matches(s1 : String, s2 : String) -> Option<String> {
    let combined = s1.chars().zip(s2.chars());
    let (l, _) : (Vec<_>,Vec<_>)= combined.filter(|elem| { elem.0 == elem.1 }).unzip();
    if l.len() == s1.len() - 1 {
        return Some(l.iter().collect());
    } else {
        return None;
    }
}

fn main() {
    match read_lines_in_file("input.txt") {
        Ok(content) => {
            println!("Day 2 pt 1 {}", calculate(content.clone()));
            println!("Day 2 pt 2 {}", find_box(content.clone()).unwrap());

        },
        Err(e) => println!("Failed to calculate pt1 {}", e)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ids_a_match() {
        assert_eq!(find_box(vec!["abcde".to_string(), 
                                "fghij".to_string(),
                                "klmno".to_string(),
                                "pqrst".to_string(),
                                "fguij".to_string(),
                                "axcye".to_string(),
                                "wvxyz".to_string()]),Some("fgij".to_string()));
    }

    #[test]
    fn test_finds_a_match() {
        assert_eq!(matches("bababc".to_string(), "badabc".to_string()),Some("baabc".to_string()));
        assert_eq!(matches("bababc".to_string(), "badebc".to_string()),None);
    }
    
    #[test]
    fn test_has_three() {
        assert_eq!(has_three("bababc"),true);
        assert_eq!(has_three("bacabc"),false);
    }
    
    #[test]
    fn test_has_two() {
        assert_eq!(has_two("bababc"),true);
        assert_eq!(has_two("abcdef"),false);
    }
    
    #[test]
    fn test_score() {
        let mut h = HashMap::<char, i32>::new();
        h.insert('a', 2);
        h.insert('b', 3);
        h.insert('c', 1);
        assert_eq!(score("bababc"), h);
    }
    
    #[test]
    fn test_calculate_list() {
        let sample = vec!["abcdef".to_string(),
                          "bababc".to_string(),
                          "abbcde".to_string(),
                          "abcccd".to_string(),
                          "aabcdd".to_string(),
                          "abcdee".to_string(),
                          "ababab".to_string()];
        assert_eq!(calculate(sample),12);
    }
}
