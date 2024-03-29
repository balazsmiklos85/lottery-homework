//extern crate time;

use std::collections::HashMap;
use std::env::{self, Args};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Error;
//use time::PreciseTime;

static DRAWN_NUMBERS: usize = 5;
static MAX_NUMBER: u8 = 90;
static MIN_NUMBER: u8 = 1;

enum ErrorCodes {
    WrongParameters,
    IoError,
    InvalidInput,
}

fn main() {
    let file_reader = FileReader::new(env::args());
    match file_reader {
        Ok(reader) => {
            let games = reader.read();
            println!("READY");
            let mut wait_for_more = true;
            let stdin = io::stdin();
            while wait_for_more {
                match stdin.lock().lines().next() {
                    Some(line_read) =>
                        wait_for_more = handle_line(&games, line_read),
                    None => wait_for_more = false // EOF?
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
                } else if converted_value < MIN_NUMBER {
                    return Err(format!("Number too low ({})",
                                       converted_value));
                }
                if converted_numbers.contains(&converted_value) {
                    return Err(format!("Number found twice ({})",
                                       converted_value));
                }
                converted_numbers.push(converted_value);
            },
            // I don't care if 5 numbers are already read, I'm not going to
            // re-implement scanf()
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

fn handle_line(games: &LotteryGames, line_read: Result<String, Error>) -> bool {
    match line_read {
        Ok(line) => {
            if line.is_empty() {
                return false;
            } else {
                //let start = PreciseTime::now();
                let winner_count = count_winners_for_line(games, line);
                winner_count.print();
                //let end = PreciseTime::now();
                //println!("Output generated in {}", start.to(end));
            }
        },
        Err(e) => {
            eprintln!("Cannot read from stdin: {}", e);
            std::process::exit(exit_code(ErrorCodes::IoError));
        }
    }
    return true;
}

fn count_winners_for_line(games: &LotteryGames, line: String) -> LotteryResult {
    match LotteryDraw::create_from_line(line) {
        Ok(draw) => return games.count_game_matches(&draw),
        Err(e) => {
            eprintln!("Invalid input: {}", e);
            // no need to exit here, but the boilerplate code in the task
            // description specified the loop to run while 5 numbers are read by
            // scanf()
            std::process::exit(exit_code(ErrorCodes::InvalidInput));
        }
    };
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
    //     -> pro: does not require thread creation as often as in
    //     count_matching_numbers(), so it shouldn't cause problems
    //     -> con: I should learn to handle threading in Rust properly to see
    //     how can I borrow match_counts mutably from the threads
    // ( ) no need to store 0 and 1 matches.
    fn count_game_matches(&self, draw: &LotteryDraw) -> LotteryResult {
        let mut winner_counts_by_match_count = LotteryResult::new();
        for game in &(self.games) {
            let matching_numbers = draw.count_matching_numbers(&game);
            winner_counts_by_match_count.increase_count_for(matching_numbers);
        }
        return winner_counts_by_match_count;
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
    use line_to_numbers;
    use LotteryGame;
    use LotteryDraw;

    fn test_line_to_numbers(line: &str) -> Result<Vec<u8>, String> {
        return line_to_numbers(line.to_string());
    }

    #[test]
    fn parsing_line_1_2_3_4_5() {
        let line = test_line_to_numbers("1 2 3 4 5");
        match line {
            Ok(numbers) => {
                assert_eq!(numbers[0], 1);
                assert_eq!(numbers[1], 2);
                assert_eq!(numbers[2], 3);
                assert_eq!(numbers[3], 4);
                assert_eq!(numbers[4], 5);
            },
            Err(_ignored) => assert!(false),
        }
    }

    #[test]
    fn parsing_line_1_2_3_4_5_foo() {
        let line = test_line_to_numbers("1 2 3 4 5 foo");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message,
                           "invalid digit found in string (foo)"),
        }
    }

    #[test]
    fn parsing_line_0_2_3_4_5() {
        let line = test_line_to_numbers("0 2 3 4 5");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message, "Number too low (0)"),
        }
    }

    #[test]
    fn parsing_line_1_2_3_4_250() {
        let line = test_line_to_numbers("1 2 3 4 250");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message, "Number too high (250)"),
        }
    }

    #[test]
    fn parsing_line_1_2_3_4_2500() {
        let line = test_line_to_numbers("1 2 3 4 500");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message,
                           "number too large to fit in target type (500)"),
        }
    }

    #[test]
    fn parsing_line_1_2_4_4_5() {
        let line = test_line_to_numbers("1 2 4 4 5");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message, "Number found twice (4)"),
        }
    }

    #[test]
    fn parsing_line_1_2_3_4_5_6() {
        let line = test_line_to_numbers("1 2 3 4 5 6");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message, "Too many numbers in line (6)"),
        }
    }

    #[test]
    fn parsing_line_1_2_3_4_foo() {
        let line = test_line_to_numbers("1 2 3 4 foo");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message,
                           "invalid digit found in string (foo)"),
        }
    }

    #[test]
    fn parsing_empty_line() {
        let line = test_line_to_numbers("");
        match line {
            Ok(_ignored) => assert!(false),
            Err(error_message) =>
                assert_eq!(error_message,
                           "cannot parse integer from empty string ()"),
        }
    }

    #[test]
    fn matching_1_2_3_4_5_with_1_2_3_4_5() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![1, 2, 3, 4, 5] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 5);
    }

    #[test]
    fn matching_1_2_3_4_5_with_1_2_3_4_6() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![1, 2, 3, 4, 6] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 4);
    }

    #[test]
    fn matching_1_2_3_4_5_with_1_2_3_7_6() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![1, 2, 3, 7, 6] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 3);
    }

    #[test]
    fn matching_1_2_3_4_5_with_1_2_8_7_6() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![1, 2, 8, 7, 6] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 2);
    }

    #[test]
    fn matching_1_2_3_4_5_with_1_9_8_7_6() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![1, 9, 8, 7, 6] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 1);
    }

    #[test]
    fn matching_1_2_3_4_5_with_10_9_8_7_6() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![10, 9, 8, 7, 6] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 0);
    }

    #[test]
    fn matching_1_2_3_4_5_with_5_4_3_2_1() {
        let game = LotteryGame { numbers: vec![1, 2, 3, 4, 5] };
        let draw = LotteryDraw { numbers: vec![5, 4, 3, 2, 1] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 5);
    }

    #[test]
    fn matching_5_4_3_2_1_with_1_2_3_4_5() {
        let game = LotteryGame { numbers: vec![5, 4, 3, 2, 1] };
        let draw = LotteryDraw { numbers: vec![1, 2, 3, 4, 5] };
        let match_count = draw.count_matching_numbers(&game);
        assert_eq!(match_count, 5);
    }
}