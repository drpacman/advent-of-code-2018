use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap};
use regex::Regex;
use std::fmt;

struct Landscape {
    contents : HashMap<(usize,usize), char>,
    max_y : usize,
    min_y : usize
}

#[derive(PartialEq, Debug)]
enum Direction {
    Down,
    Left,
    Right
}

impl Landscape {
    fn fill(&mut self) {
        while !self.add_one_level() {}
    }

    fn count_at_rest(&mut self) -> usize {
        self.fill();
        self.contents.iter().filter(|(_,v)| *v == &'~').count() as usize
    }
    
    fn count_reachable(&mut self) -> usize {
        self.fill();

        self.contents.iter().filter(|(pos,v)| (*v == &'~' || *v == &'|') && pos.1 >= self.min_y).count() as usize
    }
    
    fn add_water(&mut self, level_count : usize) -> bool {
        for i in 0..level_count {
            if self.add_one_level() {
                return true
            }
        }
        return false;
    }

    fn add_one_level(&mut self) -> bool {
        let mut pos : (usize, usize) = (500,1);
        if self.contents.contains_key(&pos) {
            println!("Landscape is full!");
            return true;
        }
        loop {
            if pos.1 == self.max_y {
                self.contents.insert(pos, '|');
                return false;
            }
            let level_entry = pos.clone();  
            let mut candidate_pos : (usize, usize) = pos.clone();        
            // find max and min x whilst looking for a way down
            let mut min_x = pos.0;
            let mut max_x = pos.0;
            let mut entry = '~';
            // look left
            'left : loop {
                // try left
                //println!("Looking Left {:?}",candidate_pos);
                match self.contents.get(&(candidate_pos.0, candidate_pos.1+1)) {
                    None => {
                        // there is a way down, go down one level
                        pos = (candidate_pos.0, candidate_pos.1+1);
                        break;
                    },
                    Some('|') => {
                        // we found a full opening going down, stop looking left
                        min_x = candidate_pos.0;
                        entry = '|';
                        break;
                    },
                    _ => {
                        if self.contents.get(&(candidate_pos.0 - 1, candidate_pos.1)) == Some(&'#') {
                            min_x = candidate_pos.0;
                            break;
                        }                
                    }
                };
                candidate_pos = (candidate_pos.0 - 1, candidate_pos.1);                
            }
            // if we didn't find a way out fill the level
            if pos.1 == level_entry.1 {
                candidate_pos = level_entry.clone();
                'right : loop {
                    //println!("Looking Right {:?}",candidate_pos);
                    match self.contents.get(&(candidate_pos.0, candidate_pos.1+1)) {
                        None => {
                            // there is a way down, go down one level
                            pos = (candidate_pos.0, candidate_pos.1+1);
                            break;
                        },
                        Some('|') => {
                            // we found a full opening going down, stop looking right
                            max_x = candidate_pos.0;
                            entry = '|';
                            break;
                        },
                        _ => {
                            if self.contents.get(&(candidate_pos.0 + 1, candidate_pos.1)) == Some(&'#') {
                                max_x = candidate_pos.0;
                                break;
                            }                
                        }
                    }
                    // try right
                    candidate_pos = (candidate_pos.0 + 1, candidate_pos.1);               
                }
            }
            // if we didn't find a way out fill the level
            if pos.1 == level_entry.1 {
                //println!("Filling Level {:?} {:?}-{:?}", pos.1, min_x, max_x);
                for x in min_x..max_x+1 {
                    self.contents.insert( (x, level_entry.1), entry );
                }
                return false;
            };            
        }
    }
}

impl std::fmt::Display for Landscape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get min/max x and y
        let mut min_x = usize::max_value();
        let mut max_x = usize::min_value();
        for (x,_) in self.contents.keys() {
            if x > &max_x { max_x = *x };
            if x < &min_x { min_x = *x };
        }
        for y in (0)..(self.max_y+2) {
            for x in (min_x-1)..(max_x+2) {
                if let Some(c) = self.contents.get(&(x,y)) {
                    write!(f,"{}",c);
                } else {
                    write!(f,".");
                }
            }
            write!(f,"\n");
        } 
        Ok(())
    }
}

fn parse_file(filename : &str) -> Result<Landscape, std::io::Error> {
    let entry_re = Regex::new(r"^([xy])=(\d+), [xy]=(\d+)\.\.(\d+)$").unwrap();
    let mut f = File::open(filename).expect("File not found");
    let mut landscape = HashMap::<(usize,usize), char>::new();
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut max_y = usize::min_value();
    let mut min_y = usize::max_value();
    for line in contents.trim().split("\n") {
        if let Some(cap) = entry_re.captures_iter(line).nth(0) {
            if &cap[1] == "y" {
                let y = cap[2].parse::<usize>().unwrap();
                if y > max_y { max_y = y };
                if y < min_y { min_y = y };
                let x1 = cap[3].parse::<usize>().unwrap();
                let x2 = cap[4].parse::<usize>().unwrap();
                for x in x1..x2+1 {
                    landscape.insert((x, y), '#');
                }
            } else {
                let x = cap[2].parse::<usize>().unwrap();
                let y1 = cap[3].parse::<usize>().unwrap();
                let y2 = cap[4].parse::<usize>().unwrap();
                if y2 > max_y { max_y = y2 };
                if y1 < min_y { min_y = y1 };
                for y in y1..y2+1 {
                    landscape.insert((x, y), '#');
                }
            }
        } 
    };
    landscape.insert((500, 0), '+');
    let result = Landscape { contents : landscape, min_y: min_y, max_y: max_y };
    println!("Loaded");
    println!("{}", result);
    println!("DONE");
    Ok(result)
}

fn main() {
    let mut landscape = parse_file("input.txt").unwrap();
    landscape.fill();
    println!("Landscape filled\n{}", landscape);
    println!("Part 1 - {}", landscape.count_reachable());
    println!("Part 2 - {}", landscape.count_at_rest());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_file() {
        let landscape = parse_file("example.txt").unwrap();
        assert_eq!(*landscape.contents.get(&(495,6)).unwrap(), '#');
    }

    #[test]
    fn test_add_water_one_drop() {
        let mut landscape = parse_file("example.txt").unwrap();
        landscape.add_water(1);
        println!("{}", landscape);
        assert_eq!(*landscape.contents.get(&(496,6)).unwrap(), '~');
    }

    #[test]
    fn test_add_water_multi_drop() {
        let mut landscape = parse_file("example.txt").unwrap();
        landscape.add_water(20);
        println!("{}", landscape);
        assert_eq!(*landscape.contents.get(&(496,6)).unwrap(), '~');
    }

    #[test]
    fn test_reachable() {
        let mut landscape = parse_file("example.txt").unwrap();
        let wet = landscape.count_reachable();
        println!("{}", landscape);        
        assert_eq!(wet, 57);
    }

    #[test]
    fn test_at_rest() {
        let mut landscape = parse_file("example.txt").unwrap();
        let wet = landscape.count_at_rest();
        println!("{}", landscape);        
        assert_eq!(wet, 29);
    }
}
