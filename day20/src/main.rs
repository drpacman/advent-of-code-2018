use std::iter::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn parse_file(filepath : &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filepath).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    contents.trim();
    Ok(contents)
}

fn measure_routes(route : &str) -> HashMap::<(i32,i32), usize> {
    let mut pos : (i32, i32) = (0,0);
    let mut prev_pos : (i32, i32) = pos;
    let mut stack = Vec::<(i32,i32)>::new();
    let mut distances = HashMap::<(i32,i32), usize>::new();
    distances.insert(pos, 0);
    let mut chars = route.chars();
    while let Some(c) = chars.next() {
        prev_pos = pos;
        match c {
            '(' => stack.push(pos),
            '|' => pos = stack[stack.len() - 1],
            ')' => pos = stack.pop().unwrap(),
            'N' | 'E' | 'S' | 'W' => {
                prev_pos = pos;
                match c {
                    'N' => { pos = (pos.0, pos.1 - 1 ); },
                    'S' => { pos = (pos.0, pos.1 + 1); },
                    'E' => { pos = (pos.0 + 1, pos.1); },
                    'W' => { pos = (pos.0 - 1, pos.1); },
                    _ => panic!()
                }
                let prev_pos_distance = *distances.get(&prev_pos).unwrap();
                match distances.get(&pos) {
                    Some(d) => {
                        distances.insert(pos, usize::min(*d, prev_pos_distance+1));                        
                    },
                    None => {
                        distances.insert(pos, prev_pos_distance+1);
                    }
                }
            },
            '^' => continue,                               
            '$' => break,
            _ => panic!("Unexpected char {}", c)         
        }
    }
    distances    
}

fn find_longest(distances: &HashMap::<(i32,i32), usize>) -> usize {
    *distances.values().into_iter().max().unwrap()
}

fn find_rooms_over_1000(distances: &HashMap::<(i32,i32), usize>) -> usize {
    let rooms : Vec<&usize> = distances.values().into_iter().filter(|d| **d >= 1000).collect();
    rooms.len()
}

fn main() {
    let path = parse_file("input.txt").unwrap();
    let distances = measure_routes(&path);
    println!("Part1 - {}", find_longest(&distances));
    println!("Part2 - {}", find_rooms_over_1000(&distances));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_measure_routes(){
        let route = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let distances = measure_routes(route);
        assert_eq!(find_longest(&distances), 31);
    }
}
