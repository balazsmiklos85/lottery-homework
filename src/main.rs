use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

enum ErrorCodes {
    WRONG_PARAMETERS,
    IO_ERROR,
}

fn main() {
    let file_name = get_input_file_name();
    let file_reader = FileReader { name: file_name };
    let games = file_reader.read();
    println!("READY");
    let mut wait_for_input = true;
    while wait_for_input {
        //TODO read line from stdin
        let draw = LotteryDraw { numbers: vec![1, 2, 3, 4, 5] };
        let results = games.count(&draw);
        results.print();
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
            std::process::exit(exit_code(ErrorCodes::WRONG_PARAMETERS));
        },
    }
}

fn exit_code(error_code: ErrorCodes) -> i32 {
    match error_code {
        ErrorCodes::WRONG_PARAMETERS => return 1,
        ErrorCodes::IO_ERROR => return 2,
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
               eprintln!("Cannot open input file: {}", self.name); 
               eprintln!("{}", e);
               std::process::exit(exit_code(ErrorCodes::IO_ERROR)); 
            },
        };
        for line in BufReader::new(input_file).lines() {
            match line {
                Ok(l) => result.add(LotteryGame::from_line(l)),
                Err(e) => {
                    eprintln!("Error while reading file: {}", self.name); 
                    eprintln!("{}", e);
                    std::process::exit(exit_code(ErrorCodes::IO_ERROR));                     
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
    numbers: HashSet<i32>,
}

impl LotteryGame {
    fn new() -> LotteryGame {
        return LotteryGame { numbers: HashSet::new() };
    }

    fn from_line(line: String) -> LotteryGame {
        let mut result = LotteryGame::new();
        //TODO: handle line
        return result;
    }
}

struct LotteryDraw {
    numbers: Vec<i32>,
}

impl LotteryDraw {
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
                Some(winners) => println!("{number:<width$} | {winner:>width$}", number=numbers_matching, winner=winners, width=6),
                None => println!("{} | {}", numbers_matching, 0),
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