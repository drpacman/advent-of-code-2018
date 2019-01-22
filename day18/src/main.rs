
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{min};
use std::fmt;

#[derive(Clone)]
struct Grid {
    round : usize,
    width : usize,
    height : usize,
    entries : Vec<Vec<char>>
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid size {} by {} at round {}\n", self.width, self.height, self.round).unwrap();
        for row in self.entries.iter() {
            for c in row.iter() {
                write!(f, "{}", c).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.entries[x][y] != other.entries[x][y] {
                    return false;
                }
            }
        }
        //println!("Equal!\n{:?}\n{:?}", self, other);
        return true;
    }
}

impl Grid {
    fn score(&self) -> usize {
        let mut counter = [0;2];
        for row in self.entries.iter() {
            for c in row.iter() {
                match c {
                    '|' => counter[0] = counter[0] + 1,
                    '#' => counter[1] = counter[1] + 1,
                    _ => ()
                }
            }
        }
        return counter[0] * counter[1];
    }

    fn update_grid(&self) -> Grid {
        let mut updated_entries : Vec<Vec<char>> = vec![];
        for _y in 0..self.height {
            let mut row = vec![];
            for _x in 0..self.width {
                row.push('.')
            }
            updated_entries.push(row);
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let mut counters = [0;3]; 
                let min_y = if y > 1 { y-1 } else { 0 };           
                let min_x = if x > 1 { x-1 } else { 0 };           
                for yy in min_y..min(self.height - 1, y+1)+1 {
                    for xx in min_x..min(self.width - 1, x+1)+1 {
                        if xx == x && yy == y {
                            continue;
                        }
                        match  self.entries[xx][yy] {
                            '.' => counters[0] = counters[0] + 1,
                            '|' => counters[1] = counters[1] + 1,
                            '#' => counters[2] = counters[2] + 1,
                            c => panic!("Unexpected character {} at {},{}", c,x,y)
                        }    
                    }
                }
                match self.entries[x][y] {
                    '.' => {
                        if counters[1] >= 3 {
                            updated_entries[x][y] = '|';
                        } else {
                            updated_entries[x][y] = '.';
                        }
                    },
                    '|' => {
                        if counters[2] >= 3 {
                            updated_entries[x][y] = '#';
                        } else {
                            updated_entries[x][y] = '|';
                        }
                    },
                    '#' => {
                        if counters[1] >= 1 && counters[2] >= 1 {
                            updated_entries[x][y] = '#';
                        } else {
                            updated_entries[x][y] = '.';
                        }
                    },
                    c => panic!("Unexpected character {} at {},{}", c,x,y)
                }
            }
        }
        Grid { round: self.round + 1, entries: updated_entries, width : self.width, height: self.height }
    }
}

fn parse_file(filename : &str) -> Result<Grid, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;  
    let mut rows : Vec<Vec<char>> = vec![];
    for row in contents.trim().split("\n") {
        rows.push( row.chars().into_iter().collect() );
    }
    let width = rows[0].len();
    let height = rows.len();
    Ok(Grid { round: 0, entries : rows, width: width, height: height })
}

fn part1(filename : &str, rounds: usize) -> usize {
    let mut grid = parse_file(filename).unwrap();
    for _round in 0..rounds {
        grid = grid.update_grid();  
    }
    println!("Part 1 result\n{:?}\n", grid);     
    grid.score()
}

fn part2(filename : &str, rounds : usize) -> usize {
    let mut grid = parse_file(filename).unwrap();
    let mut grids : Vec<Grid> = vec![];
    let mut repetition = 1;
    let mut first_in_repeating_pattern = 0;
    grids.push(grid.clone());        
    'outer : loop {
        grid = grid.update_grid();  
        for (i,g) in grids.iter().enumerate() {
            if &grid == g {
                repetition = grid.round;
                first_in_repeating_pattern = i;
                break 'outer;
            }
        }
        grids.push(grid.clone());
    }
    let loop_size = repetition - first_in_repeating_pattern;
    println!("Entry {} repeats entry {} in loops of {}", repetition, first_in_repeating_pattern, loop_size);
    if rounds >= first_in_repeating_pattern {
        let variant = (rounds - first_in_repeating_pattern) % loop_size;
        let entry = first_in_repeating_pattern + variant;
        println!("Target rounds {} is an instance of variant {} - using original entry {}", rounds, variant, entry);
        let target_grid = &grids[entry];
        println!("Part 2 result\n{:?}", target_grid);
        target_grid.score()
    } else {
        let target_grid = &grids[rounds]; 
        println!("Part 2 result (no repetition occurs in {} rounds) \n{:?}", rounds, target_grid);
        target_grid.score()       
    }
    
}

fn main() {
    println!("{:?}", parse_file("example.txt").unwrap());
    println!("Part1 {}", part1("input.txt", 10));
    println!("Part2 {}", part2("input.txt", 1000000000));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_load_grid() {
        let grid = parse_file("input.txt").unwrap();
        assert_eq!(grid.entries[1][0], '#');
    }

    #[test]
    fn test_update_grid() {
        let grid = parse_file("input.txt").unwrap();
        println!("Before\n{:?}", grid);
        let updated_grid = grid.update_grid();
        println!("{:?}", updated_grid);
        assert_eq!(updated_grid.entries[3][1], '|');
        assert_eq!(updated_grid.entries[48][48], '|');
        assert_eq!(updated_grid.entries[49][49], '.');
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt", 10),1147);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt", 417),part1("input.txt", 417));
        assert_eq!(part2("input.txt", 418),part1("input.txt", 418));
        assert_eq!(part2("input.txt", 419),part1("input.txt", 419));
        assert_eq!(part2("input.txt", 501),part1("input.txt", 501));
    }
}