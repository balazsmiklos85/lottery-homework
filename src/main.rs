use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

enum ErrorCodes {
    WRONG_PARAMETERS,
    IO_ERROR,
}

fn main() {
    let file_name = get_input_file_name();
    let file_reader = FileReader { name: file_name };
    let input = file_reader.read();
    println!("READY");
    //#include <stdio.h>
    //int main() {
    //    int a,b,c,d,e;
    //    printf("READY\n");
    //    int i = 5;
    //    while(i == 5) {
    //        i = scanf("%d %d %d %d %d", &a, &b, &c, &d, &e);
    //        if(i == 5) {
    //            printf("%d %d %d %d\n", 0, 1, 2, 3);
    //        }
    //    }
    //    return 0;
    //}
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