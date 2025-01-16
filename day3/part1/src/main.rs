
use std::io::Read;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

fn main() -> Result<()>{
    let file = File::open("input2.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = vec![];

    for (_, [num1,num2]) in regex.captures_iter(&buf).map(|c| c.extract()) {
        results.push((num1.parse::<i64>().unwrap(), num2.parse::<i64>().unwrap()));
    }

    let mut total = 0;
    for (num1,num2) in &results{
        total += num1 * num2;
    }
    println!("{}", total);

    Ok(())
}
