
use std::fs::File;
use std::io::prelude::*;

fn collapse( s : &str ) -> String {
    let mut chars = s.chars();
    let mut updated : Vec<char>= Vec::<char>::new();
    let mut curr : char = chars.next().unwrap();
    let mut recur = false;
    let mut push_last = true;
    while let Some(next) = chars.next() {
        if curr != next && curr.to_uppercase().to_string() == next.to_uppercase().to_string() {
            recur = true;
            match chars.next() {
                Some(n) => curr = n,
                None => {
                    push_last = false; 
                    ()
                }
            }
        } else {
            updated.push(curr);
            curr = next;                
        }
    }
    if push_last {
        updated.push(curr);
    }
    let u : String = updated.into_iter().collect();
    if recur {
        return collapse(&u);
    } 
    return u;
}

fn remove_and_collapse(s : &str, ignore : char) -> String {
    let cleaned : String    = s.to_string().chars().filter(|c| c.to_uppercase().next() != ignore.to_uppercase().next()).collect();
    collapse(&cleaned)
}

fn find_best(s: &str) -> String {
    let mut best = s.to_string();
    for c in vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'] {
        println!("Removing {}", c);
        let result = remove_and_collapse(&s, c).clone();
        if result.len() < best.len() {
            best = result;
        }
    }
    best
}

fn read_value_in_file(filename : &str) -> Result<String,std::io::Error> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let c = read_value_in_file("input");
    let s = c.unwrap();
    println!("Pt1 {}", collapse(&s).len());
    println!("Pt2 {}", find_best(&s).len());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_collapse(){
        assert_eq!(collapse("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
        assert_eq!(collapse("dabAcCaCBAcCcaDAa"), "dabCBAcaD");
        assert_eq!(collapse("dDabAcCaCBAcCcaDAa"), "abCBAcaD");
    }

    #[test]
    fn test_remove_and_collapse(){        
        assert_eq!(remove_and_collapse("dabAcCaCBAcCcaDA", 'a'), "dbCBcD");
        assert_eq!(remove_and_collapse("dabAcCaCBAcCcaDA", 'b'), "daCAcaDA");
        assert_eq!(remove_and_collapse("dabAcCaCBAcCcaDA", 'c'), "daDA");
        assert_eq!(remove_and_collapse("dabAcCaCBAcCcaDA", 'd'), "abCBAc");
    }

    #[test]
    fn test_find_best(){        
        assert_eq!(find_best("dabAcCaCBAcCcaDA"), "daDA");
    }
}
