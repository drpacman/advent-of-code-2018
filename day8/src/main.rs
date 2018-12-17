use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = read_contents_of_file("input.txt");
    println!("Pt1 {}", sum_of_metadata(&input));
    println!("Pt2 {}", value_of_rootnode(&input));
       
}

struct Node {
    children : Vec<Node>,
    metadata : Vec<usize>
}

impl Node {
    fn sum_metadata(&self) -> usize {
        let mut sum = self.metadata.iter().fold(0, |acc, m| acc + *m);
        for child in &self.children {
            sum = sum + child.sum_metadata()
        }
        sum
    }

    fn value_of_node(&self) -> usize {
        let mut value = 0;
        if self.children.len() == 0 {
            value = self.metadata.iter().fold(0, |acc, m| acc + *m);
        } else {
            for i in &self.metadata {
                let n = (&self.children).len();
                if i <= &n {
                    value = value + &self.children[i-1].value_of_node();
                }
            }
        }
        value
    }

}

fn read_contents_of_file(filename : &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    contents
}

fn parse_node(input : &mut std::slice::Iter<usize>) -> Node {
    let child_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();
    let mut children = vec![];
    let mut metadata = vec![];
    for _ in 0..*child_count {
        children.push(parse_node(input));
    };
    for _ in 0..*metadata_count {
        metadata.push(*input.next().unwrap())
    };
    Node {
        children: children,
        metadata: metadata
    }
}

fn parse_input(input : &str) -> Vec<usize> {
    input.to_owned().trim().split(' ').map(|s| s.parse::<usize>().unwrap()).collect()
}

fn sum_of_metadata(input : &str) -> usize {
    let values = parse_input(input);
    let mut it = values.iter();
    parse_node(&mut it).sum_metadata()
}

fn value_of_rootnode(input : &str) -> usize {
    let values = parse_input(input);
    let mut it = values.iter();
    parse_node(&mut it).value_of_node()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_metadata() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(sum_of_metadata(input), 138);
    }

    #[test]
    fn test_value_of_root_node() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(value_of_rootnode(input), 66);
    }
}