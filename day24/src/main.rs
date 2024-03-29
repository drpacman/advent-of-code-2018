use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
enum EntryType {
    ImmuneSystem,
    Infection
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Attack {
    Slashing,
    Fire,
    Radiation,
    Bludgeoning,
    Cold
}

enum BattleError {
    NoWorkDone
}

impl FromStr for Attack {
    type Err = ();
    fn from_str(input: &str) -> Result<Attack, Self::Err> {
        match input {
            "slashing"  => Ok(Attack::Slashing),
            "fire"  => Ok(Attack::Fire),
            "radiation"  => Ok(Attack::Radiation),
            "bludgeoning" => Ok(Attack::Bludgeoning),
            "cold" => Ok(Attack::Cold),
            _      => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Entry {
    units: i32,
    hit_points : i32,
    weakness : Vec<Attack>,
    immunity : Vec<Attack>,
    attack: Attack,
    damage : i32,
    initiative: i32,
    entry_type: EntryType,
    attacked: bool
}

impl Entry {
    fn effective_power(&self) -> i32 {
        return self.units * self.damage;
    }

    fn impact(&self, attack : &Attack) -> i32 {
        if self.weakness.contains(attack) {
            return 2
        } else if self.immunity.contains(attack) {
            return 0
        }
        return 1
    }
    
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        if self.effective_power() < other.effective_power() {
            return Some(Ordering::Less)
        } else if self.effective_power() > other.effective_power() {
            return Some(Ordering::Greater)
        } else {
            return Some(self.initiative.cmp(&other.initiative))
        }
    }
}

#[derive(PartialEq, Debug)]
struct Battle  {
    entrants : Vec<Entry>
}



impl Battle {

    fn boost(&self, entry_type : EntryType, boost : i32) -> Battle {
        let boosted_entrants = self.entrants.iter().map(|e| if e.entry_type == entry_type { 
            Entry {
                units: e.units,
                hit_points : e.hit_points,
                weakness : e.weakness.clone(),
                immunity : e.immunity.clone(),
                attack: e.attack.clone(),
                damage : e.damage + boost,
                initiative: e.initiative,
                entry_type: e.entry_type.clone(),
                attacked: e.attacked
            }
        } else { e.clone() } ).collect();
        Battle { entrants : boosted_entrants }
    }
    
    fn execute(&mut self) -> Result<&Self, BattleError> {
        while self.entrants.iter().find(|e| &e.entry_type != &self.entrants[0].entry_type).is_some() {
            match self.perform_battle() {
                Ok(updated) => *self=updated,
                Err(e) => return Err(e)
            }
        }    
        return Ok(self);    
    }
    
    fn sort_targetting_order(&mut self) {
        // sort for order of targetting
        self.entrants.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        self.entrants.reverse();
    }

    fn perform_battle(&mut self) -> Result<Battle, BattleError> {
        // sort for order of targetting
        self.sort_targetting_order();
        let mut targets = Vec::new();
        let mut max_initiative = 0;
        for i in 0..self.entrants.len() {
            let attacker = &self.entrants[i];
            let attack = &attacker.attack;
            max_initiative = std::cmp::max(attacker.initiative, max_initiative);
            // update target
            let mut target : Option<usize> = None;
            for j in 0..self.entrants.len() {
                let candidate = &self.entrants[j];
                if candidate.entry_type == attacker.entry_type { continue; }
                if candidate.attacked { continue; }
                match target  {
                    Some(index) => {
                        let current_target = &self.entrants[index];
                        if current_target.impact(attack) > candidate.impact(attack) { continue; }
                        else if current_target.impact(attack) == candidate.impact(attack) {
                            if current_target.effective_power() > candidate.effective_power() { continue; }
                            else if current_target.effective_power() == candidate.effective_power() {
                                if current_target.initiative > candidate.initiative { continue; }
                            }
                        }
                        target = Some(j);
                    },
                    None => {
                        if candidate.impact(attack) > 0 {
                            target = Some(j);                
                        }
                    }
                }
            }            
            // mark the target as taken
            if let Some(n) = target {
                (&mut self.entrants[n]).attacked = true;                                                
            }
        
            targets.push(target);
        }
        
        // attack targets in descending order of initiative
        let mut work_done = false;
        for initiative in (1..max_initiative+1).rev() {
            // find the entry with that inititive and update its target
            for i in 0..self.entrants.len() {
                if self.entrants[i].initiative == initiative {
                    let attacker = self.entrants[i].clone();
                    let target = targets[i];
                    match target {
                        Some(n) => {
                            let mut victim = &mut self.entrants[n];
                            let mut unit_loss : i32 = (victim.impact(&attacker.attack) * attacker.effective_power()) / victim.hit_points;
                            unit_loss = std::cmp::min(victim.units, unit_loss);
                            victim.units = victim.units - unit_loss;
                            work_done = work_done || unit_loss > 0;
                        }
                        None => {}
                    }
                }
            }
        }
        if !work_done {
           Err(BattleError::NoWorkDone)
        } else {
            Ok(Battle {
                entrants : self.entrants.iter().filter(|e| e.units > 0)
                                        .map(|e| {
                                            let mut prepared_entry = e.clone();
                                            prepared_entry.attacked = false;
                                            return prepared_entry
                                        }).collect()
            })
        }
    }
}

fn parse_file(filename : &str) -> Result<Battle, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines : Vec<String> = contents.to_string()
                    .split('\n')
                    .map(|s| s.to_owned())
                    .collect();
    let blocks : Vec<&[String]>= lines.split(|l| l.trim().is_empty() ).collect();
    let mut entrants : Vec<Entry> = vec![];
    for l in blocks[0][1..].iter() {
        entrants.push(parse_line(EntryType::ImmuneSystem, l).unwrap())
    }
    for l in blocks[1][1..].iter() {
        entrants.push(parse_line(EntryType::Infection, l).unwrap())
    }
    
    Ok(Battle { entrants })
}

fn parse_line( entry_type : EntryType, line : &str ) -> Option<Entry> {
    let entry_regex = Regex::new(r"^(\d+) units each with (\d+) hit points([a-z ,;\(\)]+)with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
    match entry_regex.captures(&line) {
        Some(capture) => {
            let units = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let hit_points = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let mut weakness : Vec<Attack>= vec![];
            let mut immunity : Vec<Attack>= vec![];
            let defences_desc = capture.get(3).unwrap().as_str().trim();
            if defences_desc.len() > 0 {
                let defences : Vec<&str>= defences_desc[1..(defences_desc.len() - 1)].split(";").collect();
                for defence in defences.iter() {
                    let words : Vec<&str> = defence.trim().split(" ").collect();
                    match words[0] {
                        "weak" => weakness = words[2..].iter().map(|&w| {
                            Attack::from_str(&str::replace(&w, ",", "")).unwrap()
                        }).collect(),
                        "immune" => immunity = words[2..].iter().map(|&w| {
                            Attack::from_str(&str::replace(&w, ",", "")).unwrap()
                        }).collect(),
                        _ => panic!("Unexpected word {}", words[0])
                    };
                }
            }
            let damage = capture.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let attack = Attack::from_str(capture.get(5).unwrap().as_str()).unwrap();
            let initiative = capture.get(6).unwrap().as_str().parse::<i32>().unwrap();
            Some(Entry { units, hit_points, weakness, immunity, attack, damage, initiative, entry_type, attacked: false })         
        },
        None => None
    }
}


fn part1() -> i32 {
    let mut battle = parse_file("/Users/caporp01/workspace/adventofcode/advent-of-code-2018/day24/input.txt").unwrap();
    match battle.execute() {
        Ok(endgame) => endgame.entrants.iter().fold(0, |sum, e| sum + e.units),
        Err(_) => panic!("Failed to resolve battle")
    }
}

fn part2() -> i32 {
    let battle = parse_file("/Users/caporp01/workspace/adventofcode/advent-of-code-2018/day24/input.txt").unwrap();
    let mut ceil = 100000;
    let mut floor = 0;
    let mut value = ceil;
    loop {
        let mut candidate = battle.boost(EntryType::ImmuneSystem, value);
        //println!("Trying value {}", value);
        match candidate.execute() {
            Ok(outcome) => {
                if outcome.entrants[0].entry_type == EntryType::ImmuneSystem {
                    ceil = value;
                    // it won. 
                    if floor < value {
                        value = floor + (value - floor)/2;
                    } else {
                        // we found our answer
                        println!("We have a winner - boost of {}", value);
                        return outcome.entrants.iter().fold(0, |sum, e| sum + e.units);
                    }
                } else {
                    // it lost - we have a new floor
                    floor = value;
                    // try half way between new floor and ceiling value
                    value = floor + (ceil - floor)/2;
                }
            },
            Err(_) => {
                // deadlock, lets try the next biggest value
                floor = value+1;
                value = floor;                
            }
        }
    }
}

fn main() {
    println!("Part1 - {}", part1());
    println!("Part2 - {}", part2());    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse_line(EntryType::ImmuneSystem, "1117 units each with 5042 hit points (weak to slashing; immune to fire, radiation, bludgeoning) with an attack that does 44 fire damage at initiative 15");
        let expected = Entry { 
            units: 1117, 
            hit_points : 5042, 
            weakness : vec![Attack::Slashing], 
            immunity : vec![Attack::Fire, Attack::Radiation, Attack::Bludgeoning],
            attack: Attack::Fire,
            damage: 44,
            initiative: 15,
            entry_type: EntryType::ImmuneSystem,
            attacked: false,
        };
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_parse_file() {
        let result = parse_file("input.txt").unwrap();
        assert_eq!(result.entrants.len(), 20);
        assert_eq!(result.entrants[2].weakness.len(), 0);
        assert_eq!(result.entrants[3].weakness[0], Attack::Slashing);
    }

    #[test]
    fn test_parse_example_file() {
        let result = parse_file("example.txt").unwrap();
        assert_eq!(result.entrants.len(), 4);
        assert_eq!(result.entrants[1].weakness.len(), 2);
        assert_eq!(result.entrants[3].immunity[0], Attack::Radiation);
    }

    #[test]
    fn test_pick_target() {
        let mut battle = parse_file("example.txt").unwrap();
        battle.sort_targetting_order();
        assert_eq!(battle.entrants[0].units, 801);
    }

    #[test]
    fn test_battle() {
        let mut battle = parse_file("example.txt").unwrap();
        battle.execute();
        assert_eq!(battle.entrants[0].units, 782);
        assert_eq!(battle.entrants[1].units, 4434);
    }    
}
