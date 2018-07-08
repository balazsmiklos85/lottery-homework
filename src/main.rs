//extern crate time;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
//use time::PreciseTime;

static DRAWN_NUMBERS: usize = 5;
static MAX_NUMBER: u8 = 90;

enum ErrorCodes {
    WrongParameters,
    IoError,
}

// TODO too long. pyramid of doom. ~5 more methods could be extracted
fn main() {
    let file_name = get_input_file_name();
    let file_reader = FileReader { name: file_name };
    let games = file_reader.read();
    println!("READY");
    let mut wait_for_more_input = true;
    let stdin = io::stdin();
    while wait_for_more_input {
        match stdin.lock().lines().next() {
            Some(result) => {
                match result {
                    Ok(line) => {
                        if line.is_empty() {
                            wait_for_more_input = false;
                        } else {
                            //let start = PreciseTime::now();
                            let draw_from_line =
                                LotteryDraw::create_from_line(line);
                            match draw_from_line {
                                Ok(draw) => games.count_game_matches(&draw)
                                                 .print(),
                                Err(e) => eprintln!("Invalid input: {}", e),
                            };
                            //let end = PreciseTime::now();
                            //println!("Output generated in {}", start.to(end));
                        }
                    },
                    Err(e) => {
                        eprintln!("Cannot read from stdin: {}", e); 
                        std::process::exit(exit_code(ErrorCodes::IoError));
                    }
                }
            } 
            None => wait_for_more_input = false // EOF?
        };        
    }
}

// TODO should be part of the FileReader maybe
fn get_input_file_name() -> String {
    let mut arguments = env::args();
    arguments.next(); // arg[0] = executable
    let result = arguments.next(); // arg[1] = input file
    match result {
        Some(r) => return r,
        None => {
            eprintln!("Wrong amount of arguments");
            eprintln!("Usage: ./lottery_homework input_file.name");
            std::process::exit(exit_code(ErrorCodes::WrongParameters));
        },
    }
    // all other arguments are just disregarded
}

// TODO would be probably unnecessary with constants in a module
fn exit_code(error_code: ErrorCodes) -> i32 {
    match error_code {
        ErrorCodes::WrongParameters => return 1,
        ErrorCodes::IoError => return 2,
    }
}

// TODO abstraction can be optimized away if necessary
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
        for input_line in BufReader::new(input_file).lines() {
            match input_line {
                Ok(line) => {
                    let game = LotteryGame::create_from_line(line);
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

// TODO abstaction can be optimized away if necessary
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

    // TODO possibly could be optimized for berrer performance
    // ( ) paralelization should make this run faster 
    //     -> pro: does not require thread creation as ofthen as in
    //     count_matching_numbers(), so it shouldn't cause problems
    //     -> con: I should learn to handle threading in Rust properly to see
    //     how can I borrow match_counts mutably from the threads
    fn count_game_matches(&self, draw: &LotteryDraw) -> LotteryResult {
        let mut match_counts = LotteryResult::new();
        for game in &(self.games) {
            let matching_numbers = draw.count_matching_numbers(&game);
            match_counts.increase_count_for(matching_numbers);
        }
        return match_counts;
    }
}

// TODO abstaction can be optimized away if necessary
struct LotteryGame {
    numbers: Vec<u8>,
}

impl LotteryGame {
    fn new() -> LotteryGame {
        return LotteryGame { numbers: Vec::new() };
    }

    fn create_from_line(line: String) -> Result<LotteryGame, String> {
        let mut result = LotteryGame::new();
        let split_line = line.split(" ");
        for number in split_line {
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

// TODO abstaction can be optimized away if necessary
struct LotteryDraw {
    numbers: Vec<u8>,
}

impl LotteryDraw {
    fn new() -> LotteryDraw {
        return LotteryDraw { numbers: Vec::new() };
    }

    fn create_from_line(line: String) -> Result<LotteryDraw, String> {
        let mut draw = LotteryDraw::new();
        let numbers = line.split(" ");
        for number in numbers {
            let integer = number.parse::<u8>();
            match integer {
                Ok(i) => {
                    if i > MAX_NUMBER {
                        return Err(format!("Number too high ({})", i));
                    }
                    draw.numbers.push(i);
                },
                Err(e) => return Err(
                            format!("Could not convert the number {}: {}",
                                number, e))
            }            
        }
        if draw.numbers.len() > DRAWN_NUMBERS {
            return Err(format!("Too many numbers drawn ({})",
                       draw.numbers.len()));
        }
        return Ok(draw);
    }

    // TODO possibly could be optimized for berrer performance
    // (x) contains() should run on HashSet faster
    //     -> no, also it kills memory usage
    // (x) contains() should run on BitSet faster 
    //     -> maybe, not supported since 1.3
    // (x) paralelization should make this run faster 
    //     -> thread/work management can take more time than the gain
    // (x) contains() should run faster on a sorted Vec with binary search
    //     -> apparently not (!?), maybe it would work better on bigger arrays
    // ( ) custom data structure? (Vec<u8> based, working like a BitSet, but
    //     hardcoded for the 5/90 lottery)
    // ( ) maybe I just didn't use rayon properly, and paralelization could
    //     still help?
    fn count_matching_numbers(&self, game: &LotteryGame) -> i32 {
        let mut matching_numbers = 0;
        for number in &(self.numbers) {
            if game.numbers.contains(&number) {
                matching_numbers += 1;
            }
        }
        return matching_numbers;
    }
}

// TODO abstaction can be optimized away if necessary
struct LotteryResult {
    // TODO possible optimization: integer indexed hashmap could basically
    // become a vector
    winner_counts_by_matches: HashMap<i32, i32>
}

impl LotteryResult {
    fn new() -> LotteryResult {
        return LotteryResult { winner_counts_by_matches: HashMap::new() };
    }

    fn print(&self) {
        let mut numbers_matching = 2;
        while numbers_matching <= 5 {
            match self.winner_counts_by_matches.get(&numbers_matching) {
                Some(winners) => print!("{} ", winners),
                None => print!("{} ", 0),
            }
            numbers_matching += 1;
        }        
        println!("");
    }

    fn increase_count_for(&mut self, matching_numbers: i32) {
        let mut new_value = 1;
        match self.winner_counts_by_matches.get(&matching_numbers) {
            Some(previous_value) => new_value += previous_value,
            None => {}
        }
        self.winner_counts_by_matches.insert(matching_numbers, new_value);
    }
}