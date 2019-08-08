use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1].parse::<String>().unwrap();
    let num = &args[2].parse::<usize>().unwrap();

    let f = File::open(filepath).unwrap();
    let mut reader = BufReader::new(f);

    let head_result = reader.lines()
        .take(*num)
        .fold("".to_string(), |mut result, x| {result.push_str(&x.unwrap());result.push_str("\n");result});


    println!("{}", head_result);
}
