

use std::io::Read;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<()>{
    let file = File::open("input1.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    
    println!("{}", buf);

    let result = buf.lines().map(|line|{
        let split: String = line.split(" ").collect();
        split 
    });

    let test:Vec<String> = result.collect();
    println!("{}", test[1]);

    let meme = is_greater_than(&test[1]);

    println!("{}", meme);

    Ok(())
}

fn is_greater_than(array: &String) -> bool{
    let mut truth = 0;
    let mut falseify= 0;
    let chars: Vec<char> = array.chars().collect();
    for i in 0..=chars.len()-1  {
        if (i < chars.len()-1) && (chars[i] > chars[i+1]){
            truth = truth + 1;
        }
        else {
           falseify = falseify + 1; 
        }
    }
    if falseify != 0 {
       return true 
    }
    else {
        return false 
    }
}
