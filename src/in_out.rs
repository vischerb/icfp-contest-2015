use super::*;

use rustc_serialize::json;
use std::path::Path;
use std::fs::File;
use std::str;
use std::io::Read;

use std::convert::AsRef;

impl Input {
    pub fn from_json<P: AsRef<Path>>(fname: P) -> Input {
        let mut temp = String::new();
        let mut file = match File::open(fname) {
            Ok(r) => r,
            Err(e) => panic!("Failed to open file with error {}", e),
        };
        file.read_to_string(&mut temp).ok().expect("Failed to read file contents.");
        let input: &str = str::from_utf8(temp.as_bytes()).ok().expect("Failed to convert &[u8] to &str???");

        let decoded: Input = match json::decode(input) {
            Ok(r) => r,
            Err(e) => panic!("Failed to decode JSON with error: {}", e),
        };
        decoded
    }
}

#[test]
fn decode_test() {
    let manual = Input{
        id:1,
        units: vec![
            Unit{
                pivot: Cell{x:0, y:0},
                members: vec![Cell{x:0, y:0}]
            }],
        width: 5,
        height: 5,
        filled: vec![Cell{x: 2, y: 4}],
        sourceLength: 100,
        sourceSeeds: vec![0],
    };
    let from_file = Input::from_json("problems/test.json");

    assert_eq!(manual, from_file);

}

impl Solution {
    pub fn to_json(&self) -> String {
        json::encode(&self).ok().expect("Couldn't turn solution into JSON for some unknowable reason.")
    }
    pub fn animate(&self, sleep_in_ms: u32) {
        use std::thread::sleep_ms;

        let states = input_to_states(&Input::from_json(format!("problems/problem_{}.json", self.problemId)));
        let mut state: State = states.iter().filter(|&s| s.seed == self.seed).next().expect("Solution has invalid seed").clone();
        println!("{}[2J", 27 as char);
        println!("Problem {}, seed {}:", self.problemId, self.seed);
        println!("{}", state.visualize());
        println!("Score: {}", state.score);

        for ch in self.solution.chars() {
            let cmd = string_to_commands(&format!("{}", ch));
            sleep_ms(sleep_in_ms);
            if sleep_in_ms != 0 {
                println!("{}[2J", 27 as char);
            }
            println!("Problem {}, seed {}, move {}, cmd {:?}:",
                     self.problemId, self.seed, ch, cmd);
            state = state.apply_sequence(&cmd);
            println!("{}", state.visualize());
            println!("Score: {}", state.score);
            if state.game_over { break; }
        }
    }

}

#[test]
fn encode_test() {
    let with_tag = Solution {
        problemId: 5,
        seed: 34,
        tag: Some("tagg".into()),
        solution: "cthulu".into()
    };
    let without_tag = Solution {
        problemId: 5,
        seed: 34,
        tag: None,
        solution: "cthulu".into()
    };
    println!("With tag: {}\n Without tag: {}.", with_tag.to_json(), without_tag.to_json());
}

pub fn submit_solutions(s: &Vec<Solution>) {
    use std::process;
    // println!("{}", json::encode(s).unwrap());
    process::Command::new("curl")
        .arg("--user").arg(":FtpwGAy9ndcLXLUlH7i96rgXLgi2SzEdym2caXEsNUI=")
        .arg("-X").arg("POST")
        .arg("-H").arg("Content-Type: application/json")
        .arg("-d").arg(json::encode(s).unwrap())
        .arg("https://davar.icfpcontest.org/teams/97/solutions")
        .spawn().unwrap().wait().unwrap();
}
