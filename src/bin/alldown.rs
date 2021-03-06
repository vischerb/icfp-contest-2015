extern crate davar;
// extern crate rustc_serialize;

use davar::*;
// use davar::Direction::*;
// use davar::Command::*;
// use rustc_serialize::json;
// use std::process;
use davar::solver::{Solver};
use std::thread;

#[allow(dead_code)]
fn main() {
    let options = opts::opts();

    let mut totalscore = 0;
    let mut solutions = Vec::new();
    for i in 0..24 {
        let mut problemscore = 0;
        let fname = format!("problems/problem_{}.json", i);
        let input = Input::from_json(fname);
        let states = input_to_states(&input);
        let num_states = states.len();
        for state in states {
            let (solution, score) = solver::AllDown::new().solve(&state, &input, &options);
            if let Some(a) = options.animate {
                solution.animate(a);
            }
            solutions.push(solution);

            totalscore += score;
            problemscore += score;
        }
        println!("problem score[{}]: {} ({} and {})", i, problemscore as f64 / num_states as f64,
                 problemscore, num_states);
        if let Some(_) = options.animate {
            thread::sleep_ms(1000);
        }
    }
    if options.submit {
        //println!("I am submitting solutions for {}.", i);
        in_out::submit_solutions(&solutions);
    }
    println!("total score: {}", totalscore);

    if !options.submit {
        println!("Not submitting solutions.");
    }
}
