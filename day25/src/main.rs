use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashSet;

fn parse_file(filename : &str) -> Result<Vec<[i32;4]>, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines : Vec<String> = contents.to_string()
                    .split('\n')
                    .map(|s| s.to_owned())
                    .collect();
    Ok(
        lines.iter().map(|line| {
            line.split(',').map(|n| {
                i32::from_str(n.trim()).unwrap()
            }).collect::<Vec<i32>>().try_into().unwrap()
        }).collect()
    )
}

fn dist(a : [i32;4], b: [i32;4]) -> i32 {
    (a[0] - b[0]).abs() +
    (a[1] - b[1]).abs() +
    (a[2] - b[2]).abs() +
    (a[3] - b[3]).abs() 
}

fn part1() -> usize {
    let input = parse_file("input.txt").unwrap();
    let mut constellations : Vec<HashSet<[i32;4]>>= Vec::new();
    
    for &entry in input.iter() {
        let mut matched : Vec<HashSet<[i32;4]>> = vec![];
        // see if matches any existing constellation
        let mut next : Vec<HashSet<[i32;4]>> = vec![];
        for constellation in constellations.iter() {
            let mut found = false;
            for e in constellation.iter() {
                let d = dist(entry, *e);
                if d <= 3 {
                    found = true;
                    break;
                }
            }

            if found {
                matched.push(constellation.clone());                    
            } else {
                next.push(constellation.clone());
            }
        }
    
        let mut constellation = if matched.len() > 0 {
            // merge all constellations that match this entry
            matched.iter().fold(HashSet::<[i32;4]>::new(), |mut res, entry| {
                res.extend(entry);
                res
            })
        } else {
            HashSet::new()
        };
        constellation.insert(entry);        
        next.push(constellation);
        constellations = next;
    }
    constellations.len()
}

fn main() {
    println!("Part 1 - {}", part1());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        match parse_file("example.txt") {
            Ok(contents) => assert_eq!(contents.len(), 8),
            Err(_) => panic!("Failed to read file")
        }
    }

    #[test]
    fn test_dist() {
        assert_eq!(dist( [1,2,3,4], [-1,-2,-3,-4]), 20 )
    }
}