//extern crate time;

use std::collections::HashMap;
use std::env::{self, Args};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
//use time::PreciseTime;

static DRAWN_NUMBERS: usize = 5;
static MAX_NUMBER: u8 = 90;

enum ErrorCodes {
    WrongParameters,
    IoError,
    InvalidInput,
}

// TODO too long. pyramid of doom. ~5 more methods could be extracted
fn main() {
    let file_reader = FileReader::new(env::args());
    match file_reader {
        Ok(reader) => {
            let games = reader.read();
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
                                        Err(e) => {
                                            eprintln!("Invalid input: {}", e);
                                            // no need to exit here, but the boilerplate
                                            // code in the task description specified
                                            // the loop to run while 5 numbers are read
                                            // by scanf()
                                            std::process::exit(
                                                exit_code(ErrorCodes::InvalidInput));
                                        }
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
        },
        Err(error_code) => {
            eprintln!("Wrong amount of arguments");
            eprintln!("Usage: ./lottery_homework input_file.name");
            std::process::exit(error_code);
        }
    }
}

// TODO would be probably unnecessary with constants in a module
fn exit_code(error_code: ErrorCodes) -> i32 {
    match error_code {
        ErrorCodes::WrongParameters => return 1,
        ErrorCodes::IoError => return 2,
        ErrorCodes::InvalidInput => return 3,
    }
}

fn line_to_numbers(line: String) -> Result<Vec<u8>, String> {
    let mut converted_numbers = Vec::new();
    let split_line = line.split(" ");
    for number in split_line {
        let converted_number = number.parse::<u8>();
        match converted_number {
            Ok(converted_value) => {
                if converted_value > MAX_NUMBER {
                    return Err(format!("Number too high ({})",
                                       converted_value));
                }
                if converted_numbers.contains(&converted_value) {
                    return Err(format!("Number found twice ({})",
                                       converted_value));
                }
                converted_numbers.push(converted_value);
            },
            Err(conversion_error) => return Err(
                format!("{} ({})", conversion_error, number))
        }
    }
    if converted_numbers.len() > DRAWN_NUMBERS {
        return Err(format!("Too many numbers in line ({})",
                           converted_numbers.len()));
    } else if converted_numbers.len() < DRAWN_NUMBERS {
        return Err(format!("Not enough numbers in line ({})",
                           converted_numbers.len()));
    }
    return Ok(converted_numbers);
}

// TODO abstraction can be optimized away if necessary
struct FileReader {
    name: String,
}

impl FileReader {
    fn new(mut arguments: Args) -> Result<FileReader, i32> {
        arguments.next(); // arg[0] = executable
        let input_file_argument = arguments.next(); // arg[1] = input file
        match input_file_argument {
            Some(input_file) => return Ok(FileReader { name: input_file }),
            None => return Err(exit_code(ErrorCodes::WrongParameters)),
        }
        // all other arguments are just disregarded
    }

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
    fn create_from_line(line: String) -> Result<LotteryGame, String> {
        match line_to_numbers(line) {
            Ok(numbers) => return Ok(LotteryGame { numbers }),
            Err(error) => return Err(error),
        }
    }
}

// TODO abstaction can be optimized away if necessary
struct LotteryDraw {
    numbers: Vec<u8>,
}

impl LotteryDraw {
    fn create_from_line(line: String) -> Result<LotteryDraw, String> {
        match line_to_numbers(line) {
            Ok(numbers) => return Ok(LotteryDraw { numbers }),
            Err(error) => return Err(error),
        }
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

#[cfg(test)]
mod tests {
    use LotteryGame;

    #[test]
    fn lottery_game_1_2_3_4_5() {
        let game = LotteryGame::create_from_line("1 2 3 4 5".to_string());
        match game {

        }
        assert_eq!(2 + 2, 4);
    }
}