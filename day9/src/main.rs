use std::collections::VecDeque;

fn main() {
    println!("Pt1 {}", play_marble_game(479,71035));
    println!("Pt2 {}", play_marble_game(479,7103500));
}

struct MarbleGame {
    board: VecDeque::<u32>
}

impl MarbleGame {
    fn new() -> MarbleGame {
        let mut m = MarbleGame {
            board : VecDeque::<u32>::new()
        };
        m.insert_marble(0);
        m
    }
    
    fn rotate_clockwise(&mut self, places : u32) {
        for _ in 0..places {
            let v = self.board.pop_back().unwrap();
            self.board.push_front(v);
        }
    }

    fn rotate_anti_clockwise(&mut self, places : u32){
        for _ in 0..places {
            let v = self.board.pop_front().unwrap();
            self.board.push_back(v);
        }
    }
    
    fn remove_marble(&mut self) -> u32 {
        self.board.pop_front().unwrap()
    }

    fn insert_marble(&mut self, marble : u32)  {
        self.board.push_front(marble)
    }
    
    fn place_marble(&mut self, marble : u32 ) -> u32 {
        if marble % 23 == 0 {
            self.rotate_anti_clockwise(7);
            let score = marble + self.remove_marble();
            self.rotate_clockwise(1);
            score
        } else {
            self.rotate_clockwise(1);
            self.insert_marble(marble);
            0
        }
    }
}

fn play_marble_game( num_players : usize, rounds : u32 ) -> u32 {
    let mut marble_game = MarbleGame::new();
    let mut score_per_player : Vec<u32> = vec!(0;num_players);
    let mut player = 0;
    (1..rounds+1).into_iter().for_each(|round| {
        score_per_player[player] = score_per_player[player] + marble_game.place_marble(round);
        player = (player + 1) % num_players
    });
    *score_per_player.iter().max().unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_insert_marble(){
        let mut marble_game = MarbleGame::new();
        let score = marble_game.place_marble(1);
        assert_eq!(score, 0);
        assert_eq!(*marble_game.board.get(0).unwrap(), 1);
        assert_eq!(*marble_game.board.get(1).unwrap(), 0);        
    }

    #[test]
    fn check_insert_marble_second_go(){
        let mut marble_game = MarbleGame::new();
        marble_game.place_marble(1);
        marble_game.place_marble(2);
        assert_eq!(*marble_game.board.get(0).unwrap(), 2);
        assert_eq!(*marble_game.board.get(1).unwrap(), 0);
        assert_eq!(*marble_game.board.get(2).unwrap(), 1); 
    }

    #[test]
    fn check_insert_marble_third_go(){
        let mut marble_game = MarbleGame::new();
        for m in 1..4 {
            marble_game.place_marble(m);
        }
        assert_eq!(*marble_game.board.get(0).unwrap(), 3);
        assert_eq!(*marble_game.board.get(1).unwrap(), 1);
        assert_eq!(*marble_game.board.get(2).unwrap(), 2); 
        assert_eq!(*marble_game.board.get(3).unwrap(), 0);        
    }

    #[test]
    fn check_insert_marble_twenty_third_go(){
        let mut marble_game = MarbleGame::new();
        for m in 1..23 {
            marble_game.place_marble(m);
        }
        assert_eq!(marble_game.place_marble(23), 32);
        assert_eq!(*marble_game.board.get(0).unwrap(), 19);
    }

    #[test]
    fn check_play_marble_game(){
        assert_eq!(play_marble_game(9,25), 32);
    }

    #[test]
    fn check_play_marble_game_2(){
        assert_eq!(play_marble_game(10,1618), 8317);
        assert_eq!(play_marble_game(30,5807), 37305);
        assert_eq!(play_marble_game(17,1104), 2764);        
    }
}
