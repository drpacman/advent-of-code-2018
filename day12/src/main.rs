use std::fs::File;
use std::io::prelude::*;

struct Rule {
    matches : String,
    action: bool
}

fn parse_file(filename : &str) -> Result<(String, Vec<Rule>), std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut lines = contents.trim().split("\n");
    let mut rules = Vec::<Rule>::new();
    let initial : String = lines.next().unwrap().split(" ").nth(2).unwrap().to_owned();
    lines.next();
    while let Some(line) = lines.next() {
        let mut elems = line.split(" ");
        let m = elems.next().unwrap().to_owned();
        let a = elems.nth(1).unwrap() == "#";
        rules.push(Rule {matches : m, action: a});
    }
    Ok((initial, rules))
}

fn apply_rules(entry : &str, rules : &Vec<Rule>) -> bool {
    for rule in rules {
        if rule.matches == entry {
            return rule.action;
        }
    }
    return false;
}

fn update_state(state : &String, offset: i64, rules : &Vec<Rule>) -> (String, i64) {
    let mut input_state : String = "....".to_owned();
    input_state.push_str(&state);
    input_state.push_str("....");
    let mut output_state = String::new();
    for pos in 0..state.len()+4 {
        let slice = &input_state[pos..pos+5];
        if apply_rules(slice, rules) {
            output_state.push('#');
        } else {
            output_state.push('.');
        }
    }
    // trim the far end
    while output_state.chars().nth(output_state.len()-1).unwrap() == '.' {
        output_state = output_state[0..output_state.len()-1].to_string();
    }
    match &output_state[0..2] {
        ".." => (output_state[2..].to_string(), offset),
        ".#" => (output_state[1..].to_string(), offset + 1),
        "#." => (output_state, offset + 2),
        "##" => (output_state, offset + 2),
        _ => (output_state, offset)
    }
}

fn pt1( state : &String, rules : &Vec<Rule> ) -> i64 {
    calculate_generation(state, rules, 20)
}

fn pt2( state : &String, rules : &Vec<Rule> ) -> i64 {
    if let Some((score, offset, diff)) = find_stable_generation(state, rules, 300) {
        return score + ((50000000000 - offset - 1) * diff);
    } 
    panic!("No stable value detected");
}

fn calculate_generation( state : &String, rules : &Vec<Rule>, generations : u32 ) -> i64 {
    let (updated_state, offset) = run_generations(state, rules, generations);
    score_generation(updated_state, offset)
}

fn run_generations( state : &String, rules : &Vec<Rule>, generations : u32 ) -> (String, i64) {
    let mut offset = 0;
    let mut updated_state = state.clone();
    for _ in 0..generations {
        let (u, o) = update_state( &updated_state, offset, &rules );
        updated_state = u;
        offset = o;
    }
    (updated_state,offset)
}

fn score_generation(state : String, offset : i64) -> i64 {
    let mut sum : i64 = 0;
    let mut pos = 0;
    for i in 0..state.len() {
        if state.chars().nth(i).unwrap() == '#' {
            pos = i as i64 - offset as i64;
            sum = sum + pos;
        }
    }
    sum
}

fn find_stable_generation( state : &String, rules : &Vec<Rule>, generations : u32 ) -> Option<(i64, i64, i64)> {
    let mut offset = 0;
    let mut updated_state = state.clone();
    let mut last_score = 0;
    let mut last_diff = 0;
    let mut same_diff_count = 0;
    for n in 0..generations {
        let (u, o) = update_state( &updated_state, offset, &rules );
        let score = score_generation(u.clone(), o);
        let diff = score - last_score;
        if last_diff == diff {
            same_diff_count = same_diff_count + 1;
        } else {
            same_diff_count = 0;
        }
        if same_diff_count > 5 {
            return Some((score, n as i64, diff));
        }
        last_diff = diff;
        last_score = score;
        updated_state = u;
        offset = o;
    }
    None
}

fn main() {
    let (initial, rules) = parse_file("input.txt").unwrap();
    println!("Part 1 - score after 20 generations is {:?}", pt1(&initial, &rules));
    println!("Part 2 - score after 50000000000 generations is {:?}", pt2(&initial, &rules));
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_update_state() {
        let (example, rules) = parse_file("example.txt").unwrap();
        let (result,offset) = update_state(&example, 0, &rules);
        let expected = "#...#....#.....#..#..#..#";
        assert_eq!(result, expected);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_run_generations() {
        let (example, rules) = parse_file("example.txt").unwrap();
        let (result,offset) = run_generations(&example, &rules, 20);
        let expected = "#....##....#####...#######....#.#..##";
        assert_eq!(result, expected);
        assert_eq!(offset, 2);
    }
}
