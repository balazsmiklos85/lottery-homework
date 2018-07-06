extern crate time;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use time::PreciseTime;

static DRAWN_NUMBERS: usize = 5;
static MAX_NUMBER: u8 = 90;

enum ErrorCodes {
    WrongParameters,
    IoError,
}

fn main() {
    let file_name = get_input_file_name();
    let file_reader = FileReader { name: file_name };
    let games = file_reader.read();
    println!("READY");
    let mut wait_for_input = true;
    let stdin = io::stdin();
    while wait_for_input {
        match stdin.lock().lines().next() {
            Some(result) => {
                match result {
                    Ok(line) => {
                        if line.is_empty() {
                            wait_for_input = false;
                        } else {
                            let start = PreciseTime::now();
                            let draw = LotteryDraw::from_line(line);
                            match draw {
                                Ok(d) => games.count(&d).print(),
                                Err(e) => eprintln!("Invalid input: {}", e),
                            };
                            let end = PreciseTime::now();
                            println!("Output generated in {}", start.to(end));
                        }
                    },
                    Err(e) => {
                        eprintln!("Cannot read from stdin: {}", e); 
                        std::process::exit(exit_code(ErrorCodes::IoError));
                    }
                }
            } 
            None => wait_for_input = false
        };        
    }
}

fn get_input_file_name() -> String {
    let mut arguments = env::args();
    arguments.next();
    let result = arguments.next();
    match result {
        Some(r) => return r,
        None => {
            eprintln!("Wrong amount of arguments");
            eprintln!("Usage: ./lottery_homework input_file.name");
            std::process::exit(exit_code(ErrorCodes::WrongParameters));
        },
    }
}

fn exit_code(error_code: ErrorCodes) -> i32 {
    match error_code {
        ErrorCodes::WrongParameters => return 1,
        ErrorCodes::IoError => return 2,
    }
}

struct FileReader {
    name: String,
}

impl FileReader {
    fn read(self) -> LotteryGames {
        let mut result = LotteryGames::new();
        let input_file = match File::open(self.name.to_string()) {
            Ok(f) => f,
            Err(e) => {
               eprintln!("Cannot open input file {}: {}", self.name, e);
               std::process::exit(exit_code(ErrorCodes::IoError)); 
            },
        };
        for line in BufReader::new(input_file).lines() {
            match line {
                Ok(l) => {
                    let game = LotteryGame::from_line(l);
                    match game {
                        Ok(g) => result.add(g),
                        Err(e) => eprintln!("Error: {}. Line ignored", e),  
                    };
                },
                Err(e) => {
                    eprintln!("Error while reading file {}: {}", self.name, e); 
                    std::process::exit(exit_code(ErrorCodes::IoError));                     
                }
            }            
        }
        return result;
    }
}

struct LotteryGames {
    games: Vec<LotteryGame>
}

impl LotteryGames {
    fn new() -> LotteryGames {
        return LotteryGames { games: Vec::new() };
    }

    fn add(&mut self, game: LotteryGame) {
        self.games.push(game);
    }

    fn count(&self, draw: &LotteryDraw) -> LotteryResult {
        let mut result = LotteryResult::new();
        for game in &(self.games) {
            result.increase(draw.count(&game));
        }
        return result;
    }
}

struct LotteryGame {
    numbers: Vec<u8>,
}

impl LotteryGame {
    fn new() -> LotteryGame {
        return LotteryGame { numbers: Vec::new() };
    }

    fn from_line(line: String) -> Result<LotteryGame, String> {
        let mut result = LotteryGame::new();
        let numbers = line.split(" ");
        for number in numbers {
            let integer = number.parse::<u8>();
            match integer {
                Ok(i) => {
                    if i > MAX_NUMBER {
                        return Err(format!("Number too high ({})", i));
                    }
                    if result.numbers.contains(&i) {
                        return Err(format!("Number found twice ({})", i));
                    }
                    result.numbers.push(i);
                },
                Err(e) => return Err(format!("{}", e))
            }            
        }
        if result.numbers.len() > DRAWN_NUMBERS {
            return Err(format!("Too many numbers in line ({})",
                result.numbers.len()));    
        }
        return Ok(result);
    }
}

struct LotteryDraw {
    numbers: Vec<u8>,
}

impl LotteryDraw {
    fn new() -> LotteryDraw {
        return LotteryDraw { numbers: Vec::new() };
    }

    fn from_line(line: String) -> Result<LotteryDraw, String> {
        let mut result = LotteryDraw::new();
        let numbers = line.split(" ");
        for number in numbers {
            let integer = number.parse::<u8>();
            match integer {
                Ok(i) => {
                    if i > MAX_NUMBER {
                        return Err(format!("Number too high ({})", i));
                    }
                    result.numbers.push(i);
                },
                Err(e) => return Err(
                            format!("Could not convert the number {}: {}",
                                number, e))
            }            
        }
        if result.numbers.len() > DRAWN_NUMBERS {
            return Err(format!("Too many numbers drawn ({})",
                       result.numbers.len()));
        }
        return Ok(result);
    }

    fn count(&self, game: &LotteryGame) -> i32 {
        let mut result = 0;
        for number in &(self.numbers) {
            if game.numbers.contains(&number) {
                result += 1;
            }
        }
        return result;
    }
}

struct LotteryResult {
    game_counts_by_matches: HashMap<i32, i32>
}

impl LotteryResult {
    fn new() -> LotteryResult {
        return LotteryResult { game_counts_by_matches: HashMap::new() };
    }

    fn print(&self) {
        println!("Numbers matching | Winners");
        let mut numbers_matching = 5;
        while numbers_matching > 1 {
            match self.game_counts_by_matches.get(&numbers_matching) {
                Some(winners) => println!("{}                | {}",
                                    numbers_matching, winners),
                None => println!("{}                | {}",
                            numbers_matching, 0),
            }
            numbers_matching -= 1;
        }        
    }

    fn increase(&mut self, matching_numbers: i32) {
        let mut new_value = 1;
        match self.game_counts_by_matches.get(&matching_numbers) {
            Some(previous_value) => new_value += previous_value,
            None => {}
        }
        self.game_counts_by_matches.insert(matching_numbers, new_value);
    }
}