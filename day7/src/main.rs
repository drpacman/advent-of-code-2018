extern crate regex;
use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Pt1 {}", pt1("input"));
    println!("Pt2 {}", pt2("input"));
}

fn read_lines_in_file(filename : &str) -> Result<Vec<String>, std::io::Error> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    let lines = contents.to_string()
                    .split('\n')
                    .filter(|s| s.to_string() != "")
                    .map(|s| s.to_owned())
                    .collect();
    Ok(lines)
}

fn parse_line( s : &str ) -> Option<(char, char)> {
    let line_regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin").unwrap();
    match line_regex.captures(&s) {
        Some(capture) => {
            let s1 = capture.at(1).unwrap();
            let s2 = capture.at(2).unwrap();
            Some((s1.chars().next().unwrap(), s2.chars().next().unwrap()))
        },
        None =>  None
    }
}

fn pt1(filename : &str) -> String {
    let contents = parse_file(filename);
    order(group_by_prerecs(contents))
}

fn pt2(filename : &str) -> u32 {
    let contents = parse_file(filename);
    let (result, t) = multiple_worker_order(group_by_prerecs(contents), 5, 60);
    t
}

fn parse_file(filename : &str) -> Vec<(char,char)> {
    read_lines_in_file(filename).unwrap().iter().map(|s| parse_line(s).unwrap()).collect()
}

fn group_by_prerecs(entries : Vec<(char, char)>) -> HashMap<char, Vec<char>> {
    let mut prerecs = HashMap::<char, Vec<char>>::new();
    let mut it = entries.iter();
    while let Some((pre, target)) = it.next() {
        prerecs.entry(*target).or_insert(vec![]).push(*pre)
    }
    // add entries with no pre-requisities
    let unavailable : Vec<char> = prerecs.keys().map(|c| *c).collect();
    let available : Vec<char> = entries.iter().filter(|e| !unavailable.contains(&e.0)).map(|e| e.0).collect();
    for a in available {
        prerecs.insert(a, vec![]);
    };
    prerecs
}

fn order(prerecs : HashMap<char, Vec<char>>) -> String {
    order_recur(prerecs, vec![], vec![], String::new())
}

fn order_recur(mut prerecs : HashMap<char, Vec<char>>, mut unlocked : Vec<char>, mut available : Vec<char>, mut current : String) -> String {
    let newly_unlocked : Vec<char> = prerecs.iter()
                                            .filter(|(_,v)| v.iter().all(|c| unlocked.contains(c)))
                                            .map(|(k,_)| *k).collect();

    for a in &newly_unlocked {
        prerecs.remove(&a);
        available.push(*a);
    }        
    available.sort();
    
    if available.len() > 0 {
        let (head, tail) = available.split_at(1);
        let c = head.iter().next().unwrap();
        let mut updated = current.clone();
        updated.push(*c);
        unlocked.push(*c);
        return order_recur(prerecs, unlocked, tail.to_vec(), updated)
    }
    current
}

fn multiple_worker_order(prerecs : HashMap<char, Vec<char>>, worker_count : usize, base_delay : u32) -> (String, u32) {
    multiple_worker_order_recur(prerecs, worker_count, vec![], base_delay, vec![], vec![], String::new(), 0)
}

fn multiple_worker_order_recur(mut prerecs : HashMap<char, Vec<char>>, 
                               worker_count : usize,
                               mut workers: Vec<(char, u32)>, 
                               base_delay : u32,
                               mut unlocked : Vec<char>, 
                               mut available : Vec<char>, 
                               current : String,
                               now : u32) -> (String, u32) {
    let newly_unlocked : Vec<char> = prerecs.iter()
                                            .filter(|(_,v)| v.iter().all(|c| unlocked.contains(c)))
                                            .map(|(k,_)| *k).collect();

    for a in &newly_unlocked {
        prerecs.remove(&a);
        available.push(*a);
    }        
    available.sort();

    while workers.len() < worker_count && available.len() > 0 {
        let c = available.remove(0);
        let delay : u32 = (c as u8 - 'A' as u8) as u32;
        workers.push((c, now + 1 + base_delay + delay));
    }

    if workers.len() > 0 {
        workers.sort_by(|x,y| x.1.cmp(&y.1));
        let (c, time) =  workers.remove(0);
        let mut updated = current.clone();
        updated.push(c);
        unlocked.push(c);
        return multiple_worker_order_recur(prerecs, worker_count, workers, base_delay, unlocked, available, updated, time)
    }
    (current, now)
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_lines() {
        assert_eq!(parse_file("input.txt"), vec![('C','A'), ('C','F'), ('A','B'),('A','D'),('B','E'),('D','E'),('F','E')]);
    }

    #[test]
    fn test_group_by_prerecs() {
        let entries = vec![('C','A'), ('C','F'), ('A','B'),('A','D'),('B','E'),('D','E'),('F','E')];
        let result : HashMap<char, Vec<char>> = group_by_prerecs(entries);
        assert_eq!(result.len(), 6);
        assert_eq!(result.get(&'C'), Some(&vec![]));
        assert_eq!(result.get(&'E'), Some(&vec!['B','D','F']));        
    }

    #[test]
    fn test_order() {
        let entries = vec![('C','A'), ('C','F'), ('A','B'),('A','D'),('B','E'),('D','E'),('F','E')];
        assert_eq!(order(group_by_prerecs(entries)), "CABDFE");
    }

    #[test]
    fn test_multi_worker_order() {
        let entries = vec![('C','A'), ('C','F'), ('A','B'),('A','D'),('B','E'),('D','E'),('F','E')];
        let (result, time) = multiple_worker_order(group_by_prerecs(entries), 2, 0);
        assert_eq!(result, "CABFDE");
        assert_eq!(time, 15);        
    }
}