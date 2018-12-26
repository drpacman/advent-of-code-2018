extern crate regex;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::ops::Index;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    pt1();
}

fn pt1() {
    let input = parse_file("input");
    let mut steps = 0;
    loop {
        let curr = step(&input, steps);
        let s = split(&curr);
        if aligned(&s) {
            println!("Aligned after {} steps!", steps);
            display(&s);
            return;
        } 
        steps = steps + 1;
    }
}

fn parse_file(filename : &str) -> Vec<((i32,i32), (i32,i32))> {
    let r= Regex::new(r"position=<\s*(\-?\d+),\s*(\-?\d+)> velocity=<\s*(\-?\d+),\s*(\-?\d+)>").unwrap();
    let mut entries : Vec<((i32,i32), (i32,i32))> = vec![];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    contents.trim().lines().for_each(|s| {
        let c = r.captures(s).unwrap();
        entries.push( ((c.index(1).parse::<i32>().unwrap(), c.index(2).parse::<i32>().unwrap()),
                        (c.index(3).parse::<i32>().unwrap(), c.index(4).parse::<i32>().unwrap())));
    });
    entries
}

fn step(state : &Vec<((i32,i32), (i32,i32))>, step : i32) -> Vec<(i32,i32)> {
    state.iter().map(|((x,y),(dx,dy))| (x+(step*(*dx)), y + (step*(*dy)))).collect()
}

fn split(state : &Vec<(i32,i32)>) -> Vec<(i32, Vec<i32>)> {
    let mut s = state.clone();
    s.sort_by(|a,b| a.1.cmp(&(b.1)));
    let mut grid : Vec<(i32, Vec<i32>)> = vec![];
    for (y, group) in s.iter().group_by(|(_,y)| *y).into_iter() {
        let mut xs : Vec<i32> = group.into_iter().map(|(x,_)| *x).collect();
        xs.sort();
        xs.dedup();
        grid.push((y, xs))
    }
    grid
}

fn display( state : &Vec<(i32, Vec<i32>)> ) {
    let min_x= state.iter().map(|(_, xs)| xs.iter().min().unwrap()).into_iter().min().unwrap();
    let max_x = state.iter().map(|(_, xs)| xs.iter().max().unwrap()).into_iter().max().unwrap();
    let min_y = state.iter().map(|(y, _)| y).into_iter().min().unwrap();
    let max_y = state.iter().map(|(y, _)| y).into_iter().max().unwrap();
    let empty_vec = Vec::<i32>::new();
    let mut x_iter = empty_vec.iter();
    let mut result = String::new();
    if max_x - min_x > 150 || max_y - min_y > 150 {
        return;
    }
    let mut entry_iter = state.iter();
    let mut next_entry = entry_iter.next();
    for y in *min_y..*max_y+1 {
        match next_entry {
            Some((next_y, xs_entry)) => {
                if y == *next_y {
                    x_iter = xs_entry.iter();
                    next_entry = entry_iter.next();
                }
            },
            None => {
                x_iter = empty_vec.iter();
            }

        }
        
        let mut next_x = x_iter.next();
        for x in *min_x..*max_x+1 {
            if Some(&x) == next_x {
                result.push('*');
                next_x = x_iter.next();
            } else {
                result.push('_');
            }
        }
        result.push('\n');        
    }
    println!("{}", result);    
}

fn aligned(state : &Vec<(i32,Vec<i32>)>) -> bool {
    let mut rows = HashMap::<i32, &Vec<i32>>::new();
    // aligned if every entry has an adjoined neighbour
    for (y, xs) in state.iter() {
        rows.insert(*y, &xs);
    }
    for (y, xs) in state.iter() {    
        for x in xs.iter() {
            if !(xs.contains(&(x+1)) || 
                 xs.contains(&(x-1)) || 
                 rows.get(&(y-1)).unwrap_or(&&vec![]).contains(&(x-1)) ||
                 rows.get(&(y-1)).unwrap_or(&&vec![]).contains(x) ||
                 rows.get(&(y-1)).unwrap_or(&&vec![]).contains(&(x+1)) ||
                 rows.get(&(y+1)).unwrap_or(&&vec![]).contains(&(x-1)) ||
                 rows.get(&(y+1)).unwrap_or(&&vec![]).contains(x) ||
                 rows.get(&(y+1)).unwrap_or(&&vec![]).contains(&(x+1))) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let input = vec![ (0,1),(0,3),(3,4),(1,4),(1,1) ];

        let expected = vec![ (1, vec![0,1]),
                             (3, vec![0]),
                             (4, vec![1,3])];
        assert_eq!(split(&input), expected);
    }

    #[test]
    fn test_aligned() {
        let input = parse_file("example.txt");
        display(&split(&step(&input, 0)));
        display(&split(&step(&input, 1)));
        display(&split(&step(&input, 2)));
        let step3 = split(&step(&input, 3));
        display(&step3);
        assert!(aligned(&step3));
    }
}

