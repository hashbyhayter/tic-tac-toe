#![allow(unused_variables)]
fn main() {
    extern crate cfg_if;
    extern crate wasm_bindgen;

    #[path = "utils.rs"]
    mod util;

    use cfg_if::cfg_if;
    use wasm_bindgen::prelude::*;
    use std::cmp;


    cfg_if! {
        // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
        // allocator.
        if #[cfg(feature = "wee_alloc")] {
            extern crate wee_alloc;
            #[global_allocator]
            static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
        }
    }
    #[wasm_bindgen]
    pub struct Game {
        board: Vec<i32>,
        player: i32,
    }

    #[wasm_bindgen(catch)]
    impl Game {
        pub fn check_status(&self) -> i32 {
            let mut won = 0;
            for n in 0..3 {
               //horizonal
               if self.board[(0 + (n * 3)) as usize] == self.board[(1 + (n * 3)) as usize] &&
                    self.board[(1 + (n * 3)) as usize] == self.board[(2 + (n * 3)) as usize] &&
                    self.board[(0 + (n * 3)) as usize] != 0 {
                    won = self.board[(0 + (n * 3)) as usize]
                }
                //vertical
                if self.board[n as usize] == self.board[(n + 3) as usize] &&
                    self.board[(n + 3) as usize] == self.board[(n + 6) as usize] &&
                    self.board[n as usize] != 0 {
                    won = self.board[n as usize]
                }
            }
            //diagonal
            if self.board[0] == self.board[4] &&
                self.board[4] == self.board[8] &&
                self.board[0] != 0 {
                won = self.board[0];
            }
            if self.board[2] == self.board[4] &&
                self.board[4] == self.board[6] &&
                self.board[2] != 0 {
                won = self.board[2];
            }
            won
        }

        fn place(&mut self, i:usize) -> Result<Vec<i32>, JsValue> {
            if i > 8 {
                return Err(JsValue::from_str("out of bounds"));
            }
            if self.board[i] > 0 {
                return Err(JsValue::from_str("already set"));
            }
            self.board[i] = self.player;
            if self.player == 1 {
                self.player = 2;
            } else {
                self.player = 1;
            }
            Ok(self.board.clone())
        }

        pub fn get_board(&self) -> Vec<i32> {
            self.board.clone()
        }
        
        pub fn play(&mut self, x:i32, y:i32) -> Result<Vec<i32>, JsValue> {
            self.place((x + (y * 3)) as usize)
        }
        
        pub fn next_turn(&mut self) -> Result<Vec<i32>, JsValue> {
            let mut moves = vec![-1000; 9];
            for n in 0..9 {
                let mut new_state = self.clone();
                if new_state.board[n as usize] == 0 {
                    let result = new_state.place(n);
                    moves[n as usize] = minimax(new_state, 10, false, self.player);
                }
            }
            let (index, value) = moves.iter().enumerate().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
            self.place(index as usize)
        }

        pub fn clone(&self) -> Game {
            let board = self.board.clone();
            let player = self.player;
            Game {
                board,
                player,
            }
        }

        pub fn new() -> Game {
            let board = vec![0; 9];
            let player = 1;
            Game {
                board,
                player,
            }
        }
    }

    fn minimax(game_state:Game, depth:i32, maximizing:bool, player:i32) -> i32 {
        let winner = game_state.check_status();
        let mut value;
        if game_state.board.iter().filter(|&x| *x == 0).count() == 0 || winner != 0 {
            if winner == 0 {
                return 0;
            } else if winner == player {
                return depth * 10;
            }
            return depth * -10;
        }
        if maximizing {
            value = -1000;
            for n in 0..9 {
                let mut new_state = game_state.clone();
                if new_state.board[n as usize] == 0 {
                    let result = new_state.place(n);
                    value = cmp::max(value, minimax(new_state, depth - 1, !maximizing, player));
                }
            }
        } else {
            value = 1000;
            for n in 0..9 {
                let mut new_state = game_state.clone();
                if new_state.board[n as usize] == 0 {
                    let result = new_state.place(n);
                    value = cmp::min(value, minimax(new_state, depth - 1, !maximizing, player));
                }
            }
        }
        value
    }
}