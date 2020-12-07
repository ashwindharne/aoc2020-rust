mod day_1;
mod day_2;
mod day_3;
mod day_4;

extern crate getopts;
extern crate reqwest;

use getopts::{HasArg, Occur, Options};
use reqwest::header;

use std::env;
use std::fs::{write, File};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.opt(
        "d",
        "day",
        "Which day's question to solve",
        "INT",
        HasArg::Yes,
        Occur::Req,
    );
    opts.opt(
        "q",
        "question",
        "Which question to solve",
        "INT",
        HasArg::Yes,
        Occur::Req,
    );
    opts.opt(
        "s",
        "session",
        "Session cookie value",
        "COOKIE",
        HasArg::Maybe,
        Occur::Optional,
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!(f.to_string())
        }
    };

    let day = matches.opt_get::<i32>("day").unwrap().unwrap();
    let question = matches.opt_get::<i32>("question").unwrap().unwrap();
    let session = match matches.opt_str("session") {
        Some(s) => format!("session={}", s),
        None => String::new(),
    };

    // Check if input exists, download if it doesn't
    let input_filename = format!("input/day_{}.txt", day);
    if !Path::new(&input_filename).exists() && session.is_empty() {
        panic!("Can't download input without session cookie argument!")
    } else if !Path::new(&input_filename).exists() {
        let client = reqwest::blocking::Client::new();
        let body = client
            .get(&format!("https://adventofcode.com/2020/day/{}/input", day))
            .header(header::COOKIE, &session)
            .send()
            .unwrap()
            .text()
            .unwrap();
        write(&input_filename, body).expect("Unable to write input file");
    }

    println!("Solving day {} question {}...", day, question);
    let file = File::open(&input_filename).unwrap();
    let reader = BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();
    println!(
        "{}",
        match (day, question) {
            (1, 1) => crate::day_1::question_1(lines.unwrap()).to_string(),
            (1, 2) => crate::day_1::question_2(lines.unwrap()).to_string(),
            (2, 1) => crate::day_2::question_1(lines.unwrap()).to_string(),
            (2, 2) => crate::day_2::question_2(lines.unwrap()).to_string(),
            (3, 1) => crate::day_3::question_1(lines.unwrap()).to_string(),
            (3, 2) => crate::day_3::question_2(lines.unwrap()).to_string(),
            (4, 1) => crate::day_4::question_1(lines.unwrap()).to_string(),
            (4, 2) => crate::day_4::question_2(lines.unwrap()).to_string(),
            _ => "Not yet implemented".to_string(),
        }
    )
}
