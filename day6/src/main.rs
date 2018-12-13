use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_lines_in_file(filename : &str) -> Result<Vec<(i32,i32)>,std::io::Error> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let lines = contents.to_string()
                        .split('\n')
                        .filter(|s| s.to_string() != "")
                        .map(|s| parse_line(s) )
                        .collect();
    Ok(lines)
}

fn parse_line(s: &str) -> (i32, i32) {
    let v : Vec<i32>= s.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();
    (v[0], v[1])
}

fn main() {
    let entries = read_lines_in_file("input").unwrap();
    println!("Pt1 {}", largest_finite(&entries));
    println!("Pt2 {}", region_size_by_distance(&entries, 10000));
}

fn distance(x1 : (i32, i32), x2 : (i32, i32)) -> i32 {
    (x1.0 - x2.0).abs() + (x1.1 - x2.1).abs()
}

fn identify_closest_item( target: (i32, i32), values : &Vec<(i32, i32)> ) -> Option<(i32, i32)> {
    let mut d : Vec<((i32,i32), i32)>= values.iter().map(|x| (*x, distance(target, *x))).collect();
    d.sort_by(|x,y| x.1.cmp(&y.1));
    if d.len() >= 2 && d[0].1 == d[1].1 {
        None
    } else {
        Some(d[0].0)
    }
}

fn identify_distance_to_all_coordinates( target: (i32, i32), values : &Vec<(i32, i32)> ) -> i32 {
    values.iter().fold(0, |sum, x| sum + distance(target, *x))
}

fn region_size_by_distance(values : &Vec<(i32, i32)>, target_distance : i32) -> i32 {
    let (xmin, xmax, ymin, ymax) = identify_bounds(&values);
    let mut region_size = 0;
    for y in (ymin - 1)..ymax+2 {
        for x in (xmin-1)..xmax+2 {
            let dist = identify_distance_to_all_coordinates((x,y), values);
            if dist < target_distance {
                region_size = region_size+1;
            }
        }
    }
    region_size
}

fn identify_bounds(values : &Vec<(i32,i32)>) -> (i32, i32, i32, i32) {
    let xs : Vec<i32>= values.iter().map(|(x,_y)| *x).collect();
    let ys : Vec<i32>= values.iter().map(|(_x,y)| *y).collect();
    (*xs.iter().min().unwrap(),
     *xs.iter().max().unwrap(), 
     *ys.iter().min().unwrap(), 
     *ys.iter().max().unwrap())
}

fn populate_grid(values : &Vec<(i32,i32)>) -> HashMap<(i32,i32),(i32,i32)> {
    let (xmin, xmax, ymin, ymax) = identify_bounds(&values);
    let mut grid = HashMap::<(i32,i32), (i32,i32)>::new();
    for y in (ymin - 1)..ymax+2 {
        for x in (xmin-1)..xmax+2 {
            match identify_closest_item((x,y), &values) {
                Some(entry) => grid.insert( (x,y), entry ),
                None => None
            };
        }
    }
    grid
}

fn identify_infinite(values : &Vec<(i32, i32)>) -> HashSet<(i32,i32)> {
    let (xmin, xmax, ymin, ymax) = identify_bounds(&values);
    let grid = populate_grid(&values);
    let mut infinite = HashSet::<(i32,i32)>::new();
    for x in xmin-1..xmax+1 {
        if let Some(entry) = grid.get(&(x,ymin-1)) {
            infinite.insert(*entry);
        }
        if let Some(entry) = grid.get(&(x,ymax+1)) {
            infinite.insert(*entry);
        }
    }
    for y in ymin-1..ymax+1 {
        if let Some(entry) = grid.get(&(xmin-1,y)) {
            infinite.insert(*entry);
        }
        if let Some(entry) = grid.get(&(xmax+1,y)) {
            infinite.insert(*entry);
        }
    }
    infinite
}

fn largest_finite(values : &Vec<(i32,i32)>) -> i32 {
    let grid = populate_grid(&values);
    let infinite = identify_infinite(values);
    let mut nearest = HashMap::<(i32,i32), i32>::new();
    for (_, neighbour) in grid {
        // don't count entries which  are infinite
        if !infinite.contains(&neighbour) {
            let v = nearest.entry(neighbour).or_insert(0);
            *v += 1;
        };
    };
    *nearest.values().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance((3,2), (4,6)),5);
    }

    #[test]
    fn test_identify_closest_item() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        assert_eq!(identify_closest_item((4,4),&values), Some((3,4))); 
        assert_eq!(identify_closest_item((4,8),&values), Some((5,5))); 
        assert_eq!(identify_closest_item((5,0),&values), None); 
    }

    #[test]
    fn test_identify_bounds() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        assert_eq!(identify_bounds(&values), (1,8,1,9));
    }

    #[test]
    fn test_populate_grid() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        let result = populate_grid(&values);        
        assert_eq!(result.get(&(0,0)), Some(&(1i32,1i32)));
        assert_eq!(result.get(&(7,6)), Some(&(5i32,5i32)));
    }

    #[test]
    fn test_identify_infinite() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        let mut infinite = HashSet::new();
        infinite.insert((1,1));
        infinite.insert((1,6));
        infinite.insert((8,3));
        infinite.insert((8,9));
        assert_eq!(identify_infinite(&values), infinite);
    }
    
    #[test]
    fn test_identify_largest_finite() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        assert_eq!(largest_finite(&values), 17);
    }

    #[test]
    fn test_identify_distance_to_all_coordinates() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        assert_eq!(identify_distance_to_all_coordinates((4,3), &values), 30);
    }

    #[test]
    fn test_region_size_by_distance() {
        let values = vec![(1, 1),(1, 6),(8, 3),(3, 4),(5, 5),(8, 9)];
        assert_eq!(region_size_by_distance(&values, 32), 16);
    }
}
