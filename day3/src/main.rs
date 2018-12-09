extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::ops::Index;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Pos {
    x : u32,
    y : u32
}

#[derive(PartialEq, Debug)]
struct Entry {
    id : u32,
    coverage : Vec<Pos>
}

#[derive(PartialEq, Debug)]
struct GridEntry {
    pos : Pos,
    ids : Vec<u32>
}

fn part1_scores_from_file(filename : &str) -> u32 {
    let entries = read_lines_in_file(filename).unwrap();
    let e : Vec<Entry> = entries.iter().map(|s| parse(s.to_string()).unwrap()).collect();
    return apply(e).values().filter(|v| v.ids.len() >= 2).count() as u32;
}

fn part2_non_overlapping_from_file(filename : &str) -> u32 {
    let entries : Vec<Entry> = read_lines_in_file(filename).unwrap().iter().map(|s| parse(s.to_string()).unwrap()).collect();
    // get the unique ids
    let ids_vec : Vec<u32> = entries.iter().map(|e : &Entry| e.id).collect();
    let ids : HashSet<u32> = HashSet::from_iter( ids_vec.iter().cloned());
    // get the ids which have an overlap
    let overlapping_ids_vec : Vec<u32>= apply(entries).values().filter(|v| v.ids.len() >= 2).map(|ge| ge.ids.clone()).flatten().collect();
    let overlapping_ids : HashSet<u32>= HashSet::from_iter(overlapping_ids_vec.iter().cloned());
    return *ids.difference(&overlapping_ids).next().unwrap();
} 

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

fn parse(value : String) -> Option<Entry> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    return match re.captures(&value) {
        Some(c) => {
            let id = c.index(1).parse::<u32>().unwrap();
            let x0 = c.index(2).parse::<u32>().unwrap();
            let y0 = c.index(3).parse::<u32>().unwrap();
            let width = c.index(4).parse::<u32>().unwrap();
            let height = c.index(5).parse::<u32>().unwrap();
            let mut coverage = Vec::<Pos>::new();
            for y in y0..(y0+height) {
                for x in x0..(x0+width) {
                    coverage.push(Pos { x: x, y: y });
                }
            }
            Some(Entry { id: id, coverage: coverage } )
        }
        _ => None
    };
}

fn apply(s : Vec<Entry>) -> HashMap<Pos,GridEntry> {
    let mut grid_entries = HashMap::<Pos, GridEntry>::new();
    let mut it = s.iter();
    while let Some(entry) = it.next() {
        for p in &entry.coverage {
            if grid_entries.contains_key(&p) {
                grid_entries.get_mut(&p).unwrap().ids.push(entry.id);
            } else {
                grid_entries.insert(p.clone(), GridEntry{ pos: p.clone(), ids : vec![entry.id]}); 
            } 
        };
    };
    return grid_entries;
}



fn main() {
    println!("{} for part1", part1_scores_from_file("input"));
    println!("{} for part2", part2_non_overlapping_from_file("input"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_apply() {
        let test = vec![Entry{ id: 1, coverage: vec![Pos{ x:1,y:2}] }, 
                        Entry {id: 2, coverage : vec![Pos{ x:1, y: 2 },Pos{ x:1, y: 3}]}];
        let mut expected = HashMap::<Pos,GridEntry>::new();
        expected.insert(Pos{ x:1, y:2}, GridEntry{ pos : Pos{ x:1, y:2}, ids: vec![1,2] });
        expected.insert(Pos{ x:1, y:3}, GridEntry{ pos : Pos{ x:1, y:3}, ids: vec![2] });        
        assert_eq!(apply(test), expected);
    }

    #[test]
    fn test_parse_entry() {
        let test = "#1 @ 871,327: 2x3";
        assert_eq!(parse(test.to_string()), Some(Entry { id: 1, coverage: vec![Pos{x:871, y: 327}, 
                                                                               Pos{x:872, y: 327}, 
                                                                               Pos{x:871, y: 328}, 
                                                                               Pos{x:872, y:328},
                                                                               Pos{x:871, y:329}, 
                                                                               Pos{x:872, y:329}]}));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1_scores_from_file("input.txt"), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_non_overlapping_from_file("input.txt"), 3);
    }
}