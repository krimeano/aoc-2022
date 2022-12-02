use std::env;
use std::panic;

enum ReaderState {
    Anything,
    Day,
}

#[derive(Debug)]
pub struct RunParams {
    pub day: u8,
    pub verbose: bool,
}

pub fn read() -> RunParams {
    let args: Vec<String> = env::args().collect();
    let mut day = 0;
    let mut verbose = false;
    let mut reader_state = ReaderState::Anything;

    for arg in args {
        match reader_state {
            ReaderState::Day => {
                match arg.parse::<u8>() {
                    Ok(x) => match x {
                        1..=25 => day = x,
                        _ => panic!("Day should be in range 1..25, received {}", x)
                    },
                    Err(e) => panic!("Can't parse a day.\nError:\n{}", e)
                }
                reader_state = ReaderState::Anything
            }
            ReaderState::Anything => {
                match arg.as_str() {
                    "-d" | "--day" => reader_state = ReaderState::Day,
                    "-v" | "--verbose" => verbose = true,
                    _ => {}
                }
            }
        }
    }

    RunParams { day, verbose }
}

