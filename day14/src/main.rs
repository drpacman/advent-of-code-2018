fn main() {
    let mut init_round = Round { receipes : &mut vec![3,7], elf_positions : &mut vec![0,1] };
    println!("Part 1 {:?}", get_score(&mut init_round, 157901));
    let mut init_round = Round { receipes : &mut vec![3,7], elf_positions : &mut vec![0,1] };
    println!("Part 2 {}", get_first_appearance(&mut init_round, vec![1,5,7,9,0,1]));
}

fn create_new_receipe(a : usize, b: usize) -> Vec<usize> {
    if a+b > 9 {
        vec![1, a + b - 10]
    } else {
        vec![a+b]
    }
}

#[derive(Debug)]
struct Round<'a> {
    receipes : &'a mut Vec<usize>,
    elf_positions : &'a mut Vec<usize>
}

fn play_round( round : &mut Round ) {
    let elf1 = round.elf_positions.iter().nth(0).unwrap();
    let elf2 = round.elf_positions.iter().nth(1).unwrap();
    let elf1_receipe = round.receipes.iter().nth(*elf1).unwrap().clone();
    let elf2_receipe = round.receipes.iter().nth(*elf2).unwrap().clone();
    let new_receipe = create_new_receipe(elf1_receipe, elf2_receipe);
    &round.receipes.extend(new_receipe);
    let len = round.receipes.len();
    *round.elf_positions = vec![ (elf1+1+elf1_receipe) % len, (elf2+1+elf2_receipe) % len ]
}

fn get_first_appearance ( round : &mut Round, seq : Vec<usize> ) -> usize {
    let target_len = seq.len();
    loop {
        play_round( round );
        let n = round.receipes.len();
        if n > target_len {
            if round.receipes[(n-target_len)..] == seq[..] {
                 return n-target_len;
            }
            if round.receipes[(n-target_len-1)..n-1] == seq[..] {
                 return n-target_len-1;
            }
        };        
    }
}

fn get_score( round : &mut Round, count : usize ) -> Vec<usize> {
    loop {
        play_round( round );
        if round.receipes.len() >= 10 + count {
            return round.receipes[count..count+10].to_vec().clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_digits() {
        assert_eq!(create_new_receipe(3,7), vec![1,0]);
        assert_eq!(create_new_receipe(2,3), vec![5]);
    }

    #[test]
    fn test_play_round() {
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        play_round(&mut round);
        assert_eq!(*round.receipes, vec![3, 7, 1, 0]);
        assert_eq!(*round.elf_positions, vec![0, 1]);
        play_round(&mut round);
        assert_eq!(*round.receipes, vec![3, 7, 1, 0, 1, 0]);
        assert_eq!(*round.elf_positions, vec![4, 3]);
    }

    #[test]
    fn test_get_score() {
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_score(&mut round, 5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
    }

    #[test]
    fn test_get_first_appearance() {
        
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_first_appearance(&mut round, vec![5,1,5,8,9]), 9);
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_first_appearance(&mut round, vec![0,1,2,4,5]), 5);
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_first_appearance(&mut round, vec![9,2,5,1,0]), 18);
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_first_appearance(&mut round, vec![5,9,4,1,4]), 2018);
    }

    #[test]
    fn test_get_first_appearance_not_terminal() {
        let mut round = Round { receipes : &mut vec![3,7], elf_positions: &mut vec![0,1] };
        assert_eq!(get_first_appearance(&mut round, vec![1,5,8,9,1]), 10);
    }
}
