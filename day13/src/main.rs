use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
struct Cart {
    pos : (usize, usize),
    direction : Direction,
    turn : i32
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn parse_file(filename : &str) -> Result<Vec<String>, std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents.split("\n").map(|s| s.to_string()).collect())
}

fn parse_initial_grid( input : Vec<String>  ) -> (Vec<String>, Vec<Cart>) {
    let mut carts = Vec::<Cart>::new();
    for y in 0..input.len() {
        let mut x = 0;
        for c in input.get(y).unwrap().chars() {
            match c {
                '^' => carts.push(Cart { pos : (x,y), direction: Direction::Up, turn : 0 }),
                'v' => carts.push(Cart { pos : (x,y), direction: Direction::Down, turn : 0 }),
                '>' => carts.push(Cart { pos : (x,y), direction: Direction::Right, turn : 0 }),
                '<' => carts.push(Cart { pos : (x,y), direction: Direction::Left, turn : 0 }),
                _ => ()
            } 
            x = x + 1;
        }
    }
    let grid : Vec<String> = input.iter().map(|row| 
        row.chars().map(|c| {
            match c {
                '^' => '|',
                'v' => '|',
                '>' => '-',
                '<' => '-',
                _ => c
            }
        }).collect()
    ).collect();
    carts.sort_by(|a,b| sort_by_pos(a.pos, b.pos));
    (grid, carts)
}

fn sort_by_pos( a: (usize, usize), b: (usize, usize) ) -> Ordering {
    if a.1 > b.1 { return Ordering::Greater; }
    if a.1 < b.1 { return Ordering::Less; }

    if a.0 > b.0 { return Ordering::Greater; }
    if a.0 < b.0 { return Ordering::Less; }

    return Ordering::Equal;
}

fn move_cart( grid : &Vec<String>, cart : &Cart ) -> Cart {
   let updated_pos = match cart.direction {
        Direction::Up => (cart.pos.0, cart.pos.1 - 1),
        Direction::Down => (cart.pos.0, cart.pos.1 + 1),
        Direction::Left => (cart.pos.0 - 1, cart.pos.1),
        Direction::Right => (cart.pos.0 + 1, cart.pos.1)
    };
    let row = grid.get(updated_pos.1).unwrap();
    let mut updated_turn = cart.turn;
    let updated_direction = match row.chars().nth(updated_pos.0).unwrap() {
        '\\' => if cart.direction == Direction::Up { Direction::Left } 
                else if cart.direction == Direction::Down { Direction::Right }
                else if cart.direction == Direction::Left { Direction::Up }
                else { Direction::Down },
        '/' =>  if cart.direction == Direction::Up { Direction::Right } 
                else if cart.direction == Direction::Down { Direction::Left }
                else if cart.direction == Direction::Left { Direction::Down }
                else { Direction::Up },
        '+' => {
                    updated_turn = (updated_turn + 1) % 3;                                                
                    if cart.turn == 0 {
                        if cart.direction == Direction::Down { Direction::Right } 
                        else if cart.direction == Direction::Up { Direction::Left }
                        else if cart.direction == Direction::Left { Direction::Down }
                        else { Direction::Up }                        
                    } else if cart.turn == 2 {
                        if cart.direction == Direction::Down { Direction::Left } 
                        else if cart.direction == Direction::Up { Direction::Right }
                        else if cart.direction == Direction::Left { Direction::Up }
                        else { Direction::Down }
                    } else {
                        cart.direction.clone()
                    }
                },
        _ => cart.direction.clone()
    };
    Cart { pos : updated_pos, direction : updated_direction, turn : updated_turn }
}

fn tick( grid : &Vec<String>, carts : &Vec<Cart>) -> (Vec<Cart>, Vec<(usize,usize)>) {    
    let mut updated_carts : Vec<Cart> = vec![];
    let mut unmoved_carts = carts.clone();
    let mut crash_locations = vec![];
    while let Some(cart) = unmoved_carts.clone().iter().next() {
        unmoved_carts.remove(0);
        let updated_cart = move_cart(&grid, cart); 
        let hit_unmoved_carts : Vec<&Cart> = unmoved_carts.iter().filter(|c| c.pos == updated_cart.pos).collect();
        let hit_moved_carts : Vec<&Cart>  = updated_carts.iter().filter(|c| c.pos == updated_cart.pos).collect();
        if hit_unmoved_carts.len() > 0 || hit_moved_carts.len() > 0 {
            unmoved_carts.retain(|c| c.pos != updated_cart.pos);
            updated_carts.retain(|c| c.pos != updated_cart.pos);
            crash_locations.push(updated_cart.pos);
        } else {
            updated_carts.push( updated_cart );
        }
    }
    updated_carts.sort_by(|a,b| sort_by_pos(a.pos, b.pos));
    (updated_carts, crash_locations)
}

fn tick_until_collision( grid : &Vec<String>, initial_carts : Vec<Cart>) -> (usize, usize) { 
    let mut carts = initial_carts;
    loop {
        let (updated_carts, crashes) = tick(&grid, &carts);
        if crashes.len() > 0 {
            return *crashes.iter().nth(0).unwrap();
        }
        carts = updated_carts;
    }
} 

fn tick_until_one_left( grid : &Vec<String>, initial_carts : Vec<Cart>) -> Option<Cart> { 
    let mut carts = initial_carts;
    let mut last_cart = None;
    while last_cart == None {
        let (updated_carts, _) = tick(&grid, &carts);
        if carts.len() == 1 {
            last_cart = Some(carts.iter().nth(0).unwrap().clone());
        }
        carts = updated_carts;
    }
    last_cart
} 

fn main() {
    let grid = parse_file("input.txt").unwrap();
    let (grid, carts) = parse_initial_grid(grid);
    println!("Part 1 {:?}", tick_until_collision(&grid, carts.clone()));
    println!("Part 2 {:?}", tick_until_one_left(&grid, carts.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let example = parse_file("example.txt").unwrap();
        let (_, carts) = parse_initial_grid(example);
        let mut carts_iter = carts.iter();
        let cart1 = carts_iter.next().unwrap();
        assert_eq!(cart1, &Cart{ pos: (2,0), direction : Direction::Right, turn: 0} );
        assert_eq!(carts_iter.next().unwrap().pos, (9,3));        
    }

    #[test]
    fn test_tick() {
        let grid = parse_file("example.txt").unwrap();
        let (grid, carts) = parse_initial_grid(grid);
        let (updated_carts, _) = tick(&grid, &carts);
        let mut carts_iter = updated_carts.iter();
        let cart1 = carts_iter.next().unwrap();
        assert_eq!(cart1.pos, (3,0));
        let cart2 = carts_iter.next().unwrap();
        assert_eq!(cart2.pos, (9,4));        
    }

    #[test]
    fn test_tick_until_collision() {
        let grid = parse_file("example.txt").unwrap();
        let (grid, carts) = parse_initial_grid(grid);
        assert_eq!(tick_until_collision(&grid, carts), (7,3));
    }
}