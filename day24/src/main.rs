use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;

fn parse_file(filename : &str) -> Result<Battle, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines : Vec<String> = contents.to_string()
                    .split('\n')
                    .map(|s| s.to_owned())
                    .collect();
    let blocks : Vec<&[String]>= lines.split(|l|  l.trim().is_empty() ).collect();
    let mut immune : Vec<Entry> = vec![];
    for l in blocks[0][1..].iter() {
        immune.push(parse_line(l).unwrap())
    }
    let mut infection : Vec<Entry> = vec![];
    for l in blocks[1][1..].iter() {
        infection.push(parse_line(l).unwrap())
    }
    
    Ok(Battle { immune, infection })
}

fn parse_line( line : &str ) -> Option<Entry> {
    let entryRegex = Regex::new(r"^(\d+) units each with (\d+) hit points([a-z ,;\(\)]+)with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
    match entryRegex.captures(&line) {
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
            Some(Entry { units, hit_points, weakness, immunity, attack, damage, initiative })         
        },
        None => None
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
    initiative: i32
}

impl Entry {
    fn effective_power(&self) -> i32 {
        return self.units * self.damage;
    }

    fn impact(&self, attack : &Attack) -> i32 {
        if self.weakness.contains(&attack) {
            return 2
        } else if !self.immunity.contains(&attack) {
            return 1
        } else {
            return 0
        }
    }
    
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        if self.effective_power() < other.effective_power() {
            return Some(Ordering::Greater)
        } else if self.effective_power() > other.effective_power() {
            return Some(Ordering::Less)
        } else {
            return Some(self.initiative.cmp(&other.initiative))
        }
    }
}


#[derive(PartialEq, Debug)]
struct Battle  {
    immune : Vec<Entry>,
    infection:  Vec<Entry>
}

#[derive(Debug)]
struct BattleEntry {
    entry : Entry,
    available : bool,
    entry_type : i32
}

fn complete_battle(battle : Battle) -> Battle {
    let mut result = battle;
    while (result.immune.len() > 0 && result.infection.len() > 0){
        result = result.perform_battle();
    }    
    return result;    
}

impl Battle {

    fn get_targetting_order(&self) -> Vec<BattleEntry> {
        let immuneEntries = self.immune.clone().into_iter().map( |e| BattleEntry { entry: e, available: true, entry_type: 0 });
        let infectionEntries = self.infection.clone().into_iter().map( |e| BattleEntry { entry: e, available: true, entry_type: 1 });
        // sort for order of targetting
        let mut entrants : Vec<BattleEntry>= infectionEntries.into_iter().chain(immuneEntries.into_iter()).collect();
        entrants.sort_by(|a, b| a.entry.partial_cmp(&b.entry).unwrap());
        return entrants; 
    }

    fn perform_battle(&mut self) -> Battle {
        // sort for order of targetting
        let mut entrants  = self.get_targetting_order();
        let mut targets = Vec::new();
        let mut max_initiative = 0;
        for i in 0..entrants.len() {
            let attacker = &entrants[i];
            let attack = attacker.entry.attack.clone();
            let effective_power = attacker.entry.effective_power();
            max_initiative = std::cmp::max(attacker.entry.initiative, max_initiative);
            // update target
            let mut target : Option<usize> = None;
            for j in 0..entrants.len() {
                let candidate = &entrants[j];
                if candidate.entry_type == attacker.entry_type { continue; }
                if !candidate.available { continue; }
                if target != None { 
                    if entrants[target.unwrap()].entry.impact(&attack) > candidate.entry.impact(&attack) { continue; }
                    else if entrants[target.unwrap()].entry.impact(&attack) == candidate.entry.impact(&attack) {
                        if entrants[target.unwrap()].entry.effective_power() > candidate.entry.effective_power() { continue; }
                        else if entrants[target.unwrap()].entry.effective_power() == candidate.entry.effective_power() {
                            if entrants[target.unwrap()].entry.initiative > candidate.entry.initiative { continue; }
                        }
                    }
                }
                target = Some(j);                
            }            
            // mark the target as taken
            if let Some(n) = target {
                (&mut entrants[n]).available = false;                                                
            }
        
            targets.push(target);
        }
        // attack targets in descending order of initiative
        let mut workDone = false;
        for initiative in (0..max_initiative + 1).rev() {
            // find the entry with that inititive and update its target
            for i in 0..entrants.len() {
                if entrants[i].entry.initiative == initiative {
                    let attacker = entrants[i].entry.clone();
                    let target = targets[i];
                    match target {
                        Some(n) => {
                            let mut victim = &mut entrants[n];
                            let mut unit_loss : i32 = (victim.entry.impact(&attacker.attack) * attacker.effective_power()) / victim.entry.hit_points;
                            unit_loss = std::cmp::min(victim.entry.units, unit_loss);
                            victim.entry.units = victim.entry.units - unit_loss;
                            workDone = workDone || unit_loss > 0;
                        }
                        None => {}//println!("No target")
                    }
                }
            }
        }
        if !workDone {
            panic!("no work done {:?}", self);
        }
        return Battle {
            immune: entrants.iter().filter(|e| e.entry_type == 0 && e.entry.units > 0).map(|e| return e.entry.clone()).collect(),
            infection: entrants.iter().filter(|e| e.entry_type == 1 && e.entry.units > 0).map(|e| return e.entry.clone()).collect()
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Attack {
    Slashing,
    Fire,
    Radiation,
    Bludgeoning,
    Cold
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
fn part1() -> i32 {
    let mut battle = parse_file("input.txt").unwrap();
    let result = complete_battle(battle);
    if result.immune.len() > 0 {
        return result.immune.iter().fold(0, |sum, e| sum + e.units);
    } else {
        return result.infection.iter().fold(0, |sum, e| sum + e.units);
    }
}

fn main() {
    println!("Part1 - {}", part1());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse_line("1117 units each with 5042 hit points (weak to slashing; immune to fire, radiation, bludgeoning) with an attack that does 44 fire damage at initiative 15");
        let expected = Entry { 
            units: 1117, 
            hit_points : 5042, 
            weakness : vec![Attack::Slashing], 
            immunity : vec![Attack::Fire, Attack::Radiation, Attack::Bludgeoning],
            attack: Attack::Fire,
            damage: 44,
            initiative: 15
        };
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_parse_file() {
        let result = parse_file("input.txt").unwrap();
        assert_eq!(result.immune.len(), 10);
        assert_eq!(result.infection.len(), 10);
        assert_eq!(result.immune[2].weakness.len(), 0);
        assert_eq!(result.immune[3].weakness[0], Attack::Slashing);
    }

    #[test]
    fn test_parse_example_file() {
        let result = parse_file("example.txt").unwrap();
        assert_eq!(result.immune.len(), 2);
        assert_eq!(result.infection.len(), 2);
        assert_eq!(result.infection[1].weakness.len(), 2);
        assert_eq!(result.immune[1].immunity[0], Attack::Fire);
    }

    #[test]
    fn test_pick_target() {
        let battle = parse_file("example.txt").unwrap();
        // pick target for infection group 2
        let attacker = &battle.get_targetting_order()[0].entry;
        assert_eq!(attacker.units, 801);
    }

    #[test]
    fn test_battle() {
        let mut battle = parse_file("example.txt").unwrap();
        let result = complete_battle(battle);
        assert_eq!(result.infection[1].units, 4434)
    }    
}
