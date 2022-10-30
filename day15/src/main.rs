use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
struct Board {
    entries : Vec<Vec<Entry>>,
    elf_count : u32,
    goblin_count : u32
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self.entries.iter().map(|row| {
            let board_row = row.iter().map(|e| match e {
                Entry::Wall => '#',
                Entry::Empty => '.',
                Entry::Goblin(_) => 'G',
                Entry::Elf(_) => 'E'
            }).collect::<String>();
            let mut score_row = Vec::new();
            row.iter().for_each(|e| match e {
                Entry::Goblin(health) => score_row.push(format!("G({})", health)),
                Entry::Elf(health) => score_row.push(format!("E({})", health)),
                _ => {}
            });
            format!("{} {}", board_row, score_row.join(","))            
        }).collect::<Vec<String>>().join("\n");
        write!(f, "Board, {} elves and {} goblins\n {}", self.elf_count, self.goblin_count, display)
    }
}

#[derive(Debug, Clone)]
enum Entry {
    Wall,
    Empty,
    Goblin(u8),
    Elf(u8)
}

fn parse_file(filename : &str) -> Result<Board, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut goblin_count = 0;
    let mut elf_count = 0;
    let entries = contents.split("\n").map(|line| line.chars().map(|e| 
        match e {
            '#' => Entry::Wall,
            '.' => Entry::Empty,
            'G' => {
                goblin_count+=1;
                Entry::Goblin(200)
            },
            'E' => {
                elf_count+=1; 
                Entry::Elf(200)
            },
            _ => panic!("Invalid item")
        }).collect()).collect();
    Ok(Board{ entries, elf_count, goblin_count })
}

fn run_battle(mut board: Board) -> (Board, u32) {
    let mut rounds = 0;
    while board.elf_count > 0 && board.goblin_count > 0 {
        let (updated_board, completed_round) = do_round(&board);
        board = updated_board;
        if completed_round {
            rounds += 1;
        }
    }
    (board, rounds)
}

fn do_round(board : &Board) -> (Board, bool) {
    // get all the moveable positions
    let max_y = board.entries.len();
    let max_x = board.entries[0].len();
    let positions : Vec<(usize, usize)>= (0..max_y).map(|y| (0..max_x).map(move |x| (x,y))).flatten().collect();
    let active_positions = positions.iter().filter(|(x,y)| match board.entries[*y][*x] {
        Entry::Goblin(_) => true,
        Entry::Elf(_) => true,
        _ => false
    });

    let mut updated_board = board.clone();
    // move each piece and run attack
    for (pos_x, pos_y) in active_positions {    
        // are we already done - skip rest of round
        if updated_board.elf_count == 0 || updated_board.goblin_count == 0 {
            return (updated_board, false)
        }
        match move_entry(updated_board, (*pos_x, *pos_y)) {
            (b, Some((mx, my))) => {
                updated_board = apply_attack(b, mx, my)
            },
            (b, None) => {
                updated_board = apply_attack(b, *pos_x, *pos_y)
            }
        }
    }
    (updated_board, true)
}

fn apply_attack(board: Board, pos_x : usize, pos_y : usize) -> Board {
    let entry = &board.entries[pos_y][pos_x];
    let neighbours = vec![(pos_x, pos_y-1), (pos_x-1, pos_y), (pos_x+1, pos_y), (pos_x, pos_y+1)];
    let mut target = None;
    let mut weakest = 201;
    for n in neighbours {
        match board.entries[n.1][n.0] {
            Entry::Goblin(health) if matches!(entry, Entry::Elf(_)) => {
                target = match target {
                    Some(_) if health >= weakest => target,
                    _ => {
                        weakest = health;
                        Some(n)
                    }
                }
            },
            Entry::Elf(health) if matches!(entry, Entry::Goblin(_)) => {
                target = match target {
                    Some(_) if health >= weakest => target,
                    _ => {
                        weakest = health;
                        Some(n)
                    }
                }
            },
            _ => {}
        }
    }
    
    match target {
        Some(t) => attack_target(board,  t),
        None => board
    }    
}

fn attack_target(board :Board, target_pos: (usize, usize)) -> Board {
    let mut entries = board.entries.clone();
    let mut elf_count = board.elf_count;
    let mut goblin_count = board.goblin_count;
    entries[target_pos.1][target_pos.0] = match entries[target_pos.1][target_pos.0] {
        Entry::Goblin(health) if health > 3 => Entry::Goblin(health - 3),
        Entry::Goblin(_) => {
            goblin_count -= 1;
            Entry::Empty
        },
        Entry::Elf(health) if health > 3 => Entry::Elf(health - 3),
        Entry::Elf(_) => {
            elf_count -= 1;
            Entry::Empty
        },
        _ => panic!("Unexpected target")
    };
    Board { entries, elf_count, goblin_count }
}

fn move_entry(board: Board, pos : (usize, usize)) -> (Board, Option<(usize, usize)>) {
    let entry = &board.entries[pos.1][pos.0];
    // explore paths to nearest 
    let mut paths : Vec<Vec<(usize, usize)>>= vec![vec![pos]];
    let mut visited = HashSet::new();
    let mut successful_paths = vec![];

    while successful_paths.len() == 0 && paths.len() > 0 {
        let mut next_paths : Vec<Vec<(usize, usize)>> = vec![];
        for path in paths.iter() {
            let (x,y) = path.last().unwrap().clone();
            let moves = vec![(x, y-1), (x-1, y), (x+1, y), (x, y+1)];
            // ignore any existing locations already visited
            let new_moves : Vec<(usize, usize)>= moves.into_iter().filter(|m| !&visited.contains(m)).collect();
            for new_move in  new_moves.iter() {
                visited.insert(new_move.clone());
                if let Some(row) = board.entries.get(new_move.1) {
                    if let Some(e) = row.get(new_move.0) {
                        match e {
                            Entry::Empty => {
                                let mut next_path = path.clone();
                                next_path.push(*new_move);
                                next_paths.push( next_path );
                            },
                            Entry::Goblin(_) if matches!(entry, Entry::Elf(_)) => {
                                successful_paths.push(path.clone());                                    
                            },
                            Entry::Elf(_) if matches!(entry, Entry::Goblin(_)) => {
                                successful_paths.push(path.clone()); 
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        paths = next_paths;
    }

    if successful_paths.len() > 0 {
        let chosen_path = successful_paths[0].clone();
        if chosen_path.len() > 1 {
            let chosen_move = chosen_path[1];
            let mut updated_entries = board.entries.clone();
            updated_entries[pos.1][pos.0] = Entry::Empty;
            updated_entries[chosen_move.1][chosen_move.0] = entry.clone();
            (Board { 
                entries: updated_entries, 
                elf_count: board.elf_count, 
                goblin_count: board.goblin_count
            }, Some(chosen_move))
        } else {
            (board, None)
        }
    } else {
        (board, None)
    }
}

fn run_scenario(name :&str) -> u32 {
    let board = parse_file(name).unwrap();
    println!("{}", board);
    let (result, rounds) = run_battle(board);
    println!("Battle complete after {} Rounds\n{}", rounds, result);
    let mut score : u32 = 0;
    result.entries.iter().for_each(|row| {
        row.iter().for_each(|entry| {
            match entry {
                Entry::Goblin(health) => score+= u32::from(*health),
                Entry::Elf(health) => score+= u32::from(*health),
                _ => ()
            }
        })
    });
    score*rounds
}
fn part1() -> u32 {
    run_scenario("input.txt")
}

fn main() {
    println!("Part1 - {}", part1());
    // println!("Part2 - {}", part2());
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_0() { assert_eq!(run_scenario("example0.txt"), 27730); }

    #[test]
    fn test_example_1() { assert_eq!(run_scenario("example1.txt"), 36334); }

    #[test]
    fn test_example_2() { assert_eq!(run_scenario("example2.txt"), 39514); }

    #[test]
    fn test_example_3() { assert_eq!(run_scenario("example3.txt"), 27755); }

    #[test]
    fn test_example_4() { assert_eq!(run_scenario("example4.txt"), 28944); }

    #[test]
    fn test_example_5() { assert_eq!(run_scenario("example5.txt"), 18740); }

}
