extern crate num;

use std::vec::Vec;
use num::pow;

pub mod simulate;

pub mod json;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    MoveW,
    MoveE,
    MoveSW,
    MoveSE,
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub id: i32,
    pub units: Vec<Unit>,
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub source_length: i32,
    pub source_seeds: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Solution {
    pub id: i32,
    pub seed: i32,
    pub tag: Option<String>,
    pub solution: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub unit_sequence: Vec<Unit>, // holds the actual sequence of units
    pub score: i32,
    pub game_over: bool,
}
impl State {
    fn new() -> State {
        State {
            width: 10,
            height: 10,
            filled: Vec::new(),
            unit_sequence: Vec::new(),
            score: 0,
            game_over: false,
        }
    }
}

pub fn input_to_states(i: Input) -> Vec<State> {
    unimplemented!()
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    let mut out = Vec::new();
    for c in s.chars() {
        match c.to_lowercase().next().unwrap() {
            'p' | '\'' | '!' | '.' | '0' | '3' => out.push(Command::MoveW),
            'b' | 'c' | 'e' | 'f' | 'y' | '2' => out.push(Command::MoveE),
            'a' | 'g' | 'h' | 'i' | 'j' | '4' => out.push(Command::MoveSW),
            'l' | 'm' | 'n' | 'o' | ' ' | '5' => out.push(Command::MoveSE),
            'd' | 'q' | 'r' | 'v' | 'z' | '1' => out.push(Command::RotateClockwise),
            'k' | 's' | 't' | 'u' | 'w' | 'x' => out.push(Command::RotateCounterClockwise),
            '\t' | '\n' | '\r' => (),
            _ => unreachable!(),
        };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_commands_works() {
        assert_eq!(string_to_commands("pack"), vec![Command::MoveW,
                                                    Command::MoveSW,
                                                    Command::MoveE,
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("PACK"), vec![Command::MoveW,
                                                    Command::MoveSW,
                                                    Command::MoveE,
                                                    Command::RotateCounterClockwise]);
        assert_eq!(string_to_commands("ei! "), vec![Command::MoveE,
                                                    Command::MoveSW,
                                                    Command::MoveW,
                                                    Command::MoveSE]);
        assert_eq!(string_to_commands("Ei! "), vec![Command::MoveE,
                                                    Command::MoveSW,
                                                    Command::MoveW,
                                                    Command::MoveSE]);
    }

	#[test]
	fn get_source_order() {
	    //seed: i32, num: i32
      let seed = 17;
      let num = 10;

      let mut out_vec: Vec<i64> = Vec::with_capacity(num as usize);

	    let modulus = 2_i64.pow(32);
	    let multiplier = 1103515245;
	    let increment = 12345;

      let mut x = seed;


      for i in 0..num {
        out_vec[i] = (multiplier*x + increment) % modulus;
        x = out_vec[i];
      }

  }
}
