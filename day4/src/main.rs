extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct GuardShift {
    id : u32,
    asleep_times : Vec<u32>
}

struct Message {
    mm : u32,
    message : String
}

fn read_sessions(filename : &str) -> Vec<Vec<String>>  {
    return split_into_sessions(read_lines_in_file(filename).unwrap());
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

fn split_into_sessions(mut lines : Vec<String>) -> Vec<Vec<String>> {
    lines.sort();
    let mut sessions : Vec<Vec<String>> = Vec::<Vec<String>>::new();
    let mut session = Vec::<String>::new();
    for line in lines {
        if line.contains("begins shift") {
            if session.len() > 0 {
                sessions.push(session);
            }
            session = Vec::<String>::new();
        }
        session.push(line);
    }
    if session.len() > 0 {
        sessions.push(session);
    }            
    return sessions;
}

fn parse(sessions : Vec<Vec<String>>) -> Vec<GuardShift> {
    // [1518-11-01 00:00] Guard #10 begins shift
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:05] wakes up
    let mut shifts = Vec::<GuardShift>::new();
    for session in sessions {
        let mut last_time = 0;
        let mut awake = true;
        let mut asleep_times = Vec::<u32>::new();
        let mut guard_id : Option<u32>= None;

        for entry in session {
            match parse_line(&entry) {
                Some(cmd) => {
                    let m = cmd.message.to_owned();
                    if m == "falls asleep" {
                        awake = false;
                        last_time = cmd.mm;
                    } else if m == "wakes up" {
                        awake = true;
                        asleep_times.extend( last_time..cmd.mm )
                    } else {
                        let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                        if let Some(c) = guard_regex.captures(&m) {
                           guard_id = Some(c.index(1).parse::<u32>().unwrap());                            
                        }
                    };
                },
                None => ()
            }
        }
        if !awake {
            asleep_times.extend( last_time..60 )
        }    
        shifts.push(GuardShift{ id: guard_id.unwrap(), asleep_times: asleep_times });    
    };
    return shifts;              
}

fn parse_line(str : &String) -> Option<Message> {
    let line_regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.*)").unwrap();
    match line_regex.captures(&str) {
        Some(capture) => {
            let mm = capture.index(1).parse::<u32>().unwrap();
            let message = capture.at(2).unwrap().to_string();
            let result = Message{ mm : mm, message: message};
            return Some(result);
        },
        None =>  return None
    }
}

fn sleepiest_guard(shifts : &Vec<GuardShift>) -> u32 {
    let mut counter = HashMap::<u32, u32>::new();
    for shift in shifts {
        if counter.contains_key(&shift.id) {
            let update = counter.get(&shift.id).unwrap() + shift.asleep_times.len() as u32;
            counter.insert(shift.id, update);
        } else {
            counter.insert(shift.id, shift.asleep_times.len() as u32);
        }
    }
    return *counter.iter().max_by(|entryA, entryB| entryA.1.cmp(&entryB.1)).unwrap().0;
}

fn asleep_minute_frequency(shifts : &Vec<GuardShift>, id: u32) -> HashMap<u32, u32> {
    let mut times = Vec::<u32>::new();
    for shift in shifts.iter().filter(|shift| shift.id == id) {
        times.extend(&shift.asleep_times);
    }
    let mut counter = HashMap::<u32, u32>::new();
    for time in times {
        if counter.contains_key(&time) {
            let update = counter.get(&time).unwrap() + 1;
            counter.insert(time, update);
        } else {
            counter.insert(time, 1);
        }
    }
    return counter;
}
fn sleepiest_minute(shifts : &Vec<GuardShift>, id: u32) -> u32 {
    let mut counter = asleep_minute_frequency(shifts, id);
    return *counter.iter().max_by(|entryA, entryB| entryA.1.cmp(&entryB.1)).unwrap().0;
}

fn strategy_one(filename : &str) -> u32 {
    let shifts = parse(read_sessions(filename));
    let sleepiest_guard = sleepiest_guard(&shifts);
    println!("Sleepiest guard is {}", sleepiest_guard);
    return sleepiest_guard * sleepiest_minute(&shifts, sleepiest_guard);
}


fn strategy_two(filename : &str) -> u32 {
    let shifts = parse(read_sessions(filename));
    // get all the guard ids
    let mut guard_ids : Vec<u32> = shifts.iter().map(|shift| shift.id).collect();
    guard_ids.sort();
    guard_ids.dedup();
    let mut target_guard : u32 = 0;
    let mut target_minute : u32 = 0;
    let mut max_freq : u32 = 0;
    for id in guard_ids {
        let mut counter : HashMap<u32, u32> = asleep_minute_frequency(&shifts, id);
        if let Some((minute, freq)) = counter.iter().max_by(|entryA, entryB| entryA.1.cmp(&entryB.1)) {
            if *freq > max_freq {
                max_freq = *freq;
                target_minute = *minute;
                target_guard = id;
            }
        }
    }
    return target_guard * target_minute;
}

fn main() {
    println!("Pt1 {}", strategy_one("input"));
    println!("Pt2 {}", strategy_two("input"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_sleeping_times(){
        assert_eq!(read_sessions("input.txt").len(), 6);
    }

    #[test]
    fn test_parse(){
        let sessions = read_sessions("input.txt");
        let shifts = parse(sessions);
        assert_eq!(shifts[0].id, 10);
        assert_eq!(shifts[0].asleep_times.len(), 45);
        assert_eq!(shifts[5].id, 1);
        assert_eq!(shifts[5].asleep_times.len(), 15);        
    }

    #[test]
    fn test_sleepiest_guard(){
        let sessions = read_sessions("input.txt");
        let shifts = parse(sessions);
        assert_eq!(sleepiest_guard(&shifts), 10);
    }

    #[test]
    fn test_sleepiest_minute(){
        let sessions = read_sessions("input.txt");
        let shifts = parse(sessions);
        assert_eq!(sleepiest_minute(&shifts, 10), 24);
    }

    #[test]
    fn test_strategy_one(){
        assert_eq!(strategy_one("input.txt"), 240);
    }

    #[test]
    fn test_strategy_two(){
        assert_eq!(strategy_two("input.txt"), 4455);
    }

}
