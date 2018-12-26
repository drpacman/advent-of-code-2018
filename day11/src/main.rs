use std::collections::HashMap;
fn calculate_power(x : u32, y : u32, serial_number : u32) -> i32 {
    let rack_id=x+10;
    let power_level = (((rack_id)*y) + serial_number) * rack_id;
    let tmp = power_level.to_string();
    if tmp.len() >= 3 {
        let n = tmp.chars().nth(tmp.len() - 3).unwrap().to_digit(10).unwrap() as i32;
        n - 5
    } else {
        -5
    }
}

fn max_grid_pos(serial_number: u32, size : u32, memo : &HashMap::<(u32,u32), i32>) -> ((u32, u32), i32, HashMap::<(u32,u32), i32>) {
    let mut max = 0;
    let mut result = (0,0);
    let mut power = HashMap::<(u32,u32), i32>::new();
    let mut grid_value = 0;
    for x in 1..(300 - size) {
        for y in 1..(300-size) {
            match memo.get(&(x,y)) {
                Some(g) => {
                    grid_value = *g;
                    for dx in 0..size {
                        grid_value = grid_value + calculate_power(x+dx, y+size-1, serial_number);
                    }
                    for dy in 0..size-1 {
                        grid_value = grid_value + calculate_power(x+size-1, y+dy, serial_number);
                    }
                },
                None => {
                    grid_value = 0;
                    for dx in 0..size {
                        for dy in 0..size {
                            grid_value = grid_value + calculate_power(x+dx, y+dy, serial_number);
                        }
                    }
                }
            };
            power.insert((x,y), grid_value);
            if grid_value > max {
               result = (x,y);
               max = grid_value;
            }
        }
    }
    (result, max, power)
}

fn max_grid_pos_any_size(serial_number: u32) -> ((u32, u32), u32) {
    let mut max = 0;
    let mut result = (0,0);
    let mut grid_size = 0;
    let mut memo = HashMap::<(u32,u32), i32>::new();
    for n in 1..300 {
        let (r, m, updated) = max_grid_pos(serial_number, n, &memo);
        memo = updated;
        if m >= max {
          result = r;
          max = m;
          grid_size = n;
        } else {
          return (result, grid_size)
        }
    }
    (result, grid_size)    
}

fn main() {
    let (r,_,_) = max_grid_pos(6548, 3, &HashMap::<(u32,u32), i32>::new());
    println!("Part 1 - {:?}", r);
    println!("Part 2 - {:?}", max_grid_pos_any_size(6548));
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_calculate_power(){
        assert_eq!(calculate_power(3,5,8), 4);
        assert_eq!(calculate_power(122,79,57), -5);
        assert_eq!(calculate_power(217,196,39), 0);
        assert_eq!(calculate_power(101,153,71), 4);
    }

    #[test]
    fn test_max_grid(){
        let (r,m,_) = max_grid_pos(18, 3, &HashMap::new());
        assert_eq!(r, (33,45));    
        assert_eq!(m, 29);        
    }

    #[test]
    fn test_max_grid_pos_any_size(){
        let (r,n) = max_grid_pos_any_size(18);
        assert_eq!(r, (90,269));    
        assert_eq!(n, 16);        
    }
}
